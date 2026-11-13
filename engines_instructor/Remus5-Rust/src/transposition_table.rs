//! Remember searched positions in four-entry, 64-byte buckets.
//!
//! The table holds 512 MiB of buckets, indexed with multiply-shift hashing. A full bucket replaces its
//! least useful entry according to search age and depth.

use std::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};
use std::ffi::c_void;
use std::ptr;

use crate::board::{MATE_SCORE, Move};

pub const EXACT: u8 = 0;
pub const LOWER: u8 = 1;
pub const UPPER: u8 = 2;

const TABLE_BYTES: usize = 536_870_912; // Fixed 512 MiB cache
const BUCKET_COUNT: usize = TABLE_BYTES / size_of::<Bucket>();
const MEM_COMMIT: u32 = 0x1000; // Back the region with Windows commit capacity
const MEM_RESERVE: u32 = 0x2000; // Reserve addresses for the table
const MEM_RELEASE: u32 = 0x8000; // Release the whole allocation on drop
const PAGE_READWRITE: u32 = 0x04; // Ordinary writable, nonexecutable pages

#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualAlloc(
        address: *mut c_void,
        bytes: usize,
        allocation: u32,
        protect: u32,
    ) -> *mut c_void;
    fn VirtualFree(address: *mut c_void, bytes: usize, free_type: u32) -> i32;
}
const ENTRIES_PER_BUCKET: usize = 4;
const AGE_LIMIT: u8 = 64; // Ages wrap around at this count of searches
const AGE_WEIGHT: i32 = 8; // One search of age counts as much as this many plies

#[derive(Clone, Copy)]
struct Entry {
    key: u64, // 0 marks an empty entry
    score: i32,
    mv: Move,
    depth: u8,
    bound_and_age: u8, // The bound in the low 2 bits and the age above them
}

const _: () = assert!(size_of::<Entry>() == 16 && size_of::<Bucket>() == 64);

#[repr(align(64))]
#[derive(Clone, Copy)]
struct Bucket {
    entries: [Entry; ENTRIES_PER_BUCKET],
}

/// Map the whole 64-bit key range onto the fixed table.
#[inline]
fn bucket_index(key: u64) -> usize {
    ((u128::from(key) * BUCKET_COUNT as u128) >> 64) as usize
}

/// Obtain aligned zero-filled pages without touching the whole table at startup.
fn allocate_buckets() -> *mut Bucket {
    // Windows aligns the base to at least a page boundary, exceeding Bucket's 64
    // bytes. Integer-only entries accept zero bits. Pages are supplied on access.
    unsafe {
        VirtualAlloc(
            ptr::null_mut(),
            TABLE_BYTES,
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        )
        .cast()
    }
}

/// What probe() found: the depth, score, bound, and best move of a position.
pub struct Stored {
    pub depth: i32,
    pub score: i32,
    pub bound: u8,
    pub mv: Move,
}

pub struct Table {
    buckets: *mut Bucket,
    age: u8,
}

impl Table {
    pub fn new() -> Table {
        // A null allocation disables caching while leaving native search available.
        Table {
            buckets: allocate_buckets(),
            age: 0,
        }
    }

    pub fn size_mib(&self) -> usize {
        if self.buckets.is_null() {
            0
        } else {
            TABLE_BYTES / (1024 * 1024)
        }
    }

    /// Mark the start of a new search, so older entries give way first.
    pub fn new_search(&mut self) {
        self.age = (self.age + 1) % AGE_LIMIT;
    }

    /// Save one search result. A mate score is stored as mate from here.
    #[inline]
    pub fn store(
        &mut self,
        key: u64,
        depth: i32,
        mut score: i32,
        bound: u8,
        mv: Move,
        ply: i32,
    ) -> bool {
        if score > MATE_SCORE - 1000 {
            score += ply;
        } else if score < -MATE_SCORE + 1000 {
            score -= ply;
        }
        if self.buckets.is_null() || key == 0 {
            return false; // Zero marks an empty entry, so this key is not cached.
        }
        let age = self.age;
        let index = bucket_index(key);
        // The index is inside the owned allocation; &mut self excludes other borrows.
        let entries = unsafe { &mut (*self.buckets.add(index)).entries };
        let mut chosen = 0;
        let mut lowest_value = i32::MAX;
        for (i, entry) in entries.iter().enumerate() {
            if entry.key == key || entry.key == 0 {
                chosen = i;
                break;
            }
            let searches_ago = i32::from(age.wrapping_sub(entry.bound_and_age >> 2) % AGE_LIMIT);
            let value = i32::from(entry.depth) - AGE_WEIGHT * searches_ago;
            if value < lowest_value {
                lowest_value = value;
                chosen = i;
            }
        }
        let replacement = entries[chosen].key != 0 && entries[chosen].key != key;
        entries[chosen] = Entry {
            key,
            score,
            mv,
            depth: depth as u8,
            bound_and_age: bound | (age << 2),
        };
        replacement
    }

    /// Ask the processor to start loading the bucket of a position. The table is far
    /// larger than the processor's cache, so a bucket takes about as long to arrive
    /// from memory as a whole position takes to search; asking as soon as the key is
    /// known lets the search work during that wait.
    #[inline(always)]
    pub fn prefetch(&self, key: u64) {
        if !self.buckets.is_null() {
            let bucket = self.buckets.wrapping_add(bucket_index(key));
            // A prefetch reads nothing and cannot fault, whatever the address.
            unsafe {
                _mm_prefetch::<_MM_HINT_T0>(bucket.cast::<i8>());
            }
        }
    }

    /// Return what the table holds for the position, or None.
    #[inline]
    pub fn probe(&self, key: u64, ply: i32) -> Option<Stored> {
        if self.buckets.is_null() || key == 0 {
            return None;
        }
        let index = bucket_index(key);
        // The allocation stays live while self is borrowed, and index is in bounds.
        let entries = unsafe { &(*self.buckets.add(index)).entries };
        for entry in entries {
            if entry.key == key {
                let mut score = entry.score;
                if score > MATE_SCORE - 1000 {
                    score -= ply;
                } else if score < -MATE_SCORE + 1000 {
                    score += ply;
                }
                return Some(Stored {
                    depth: i32::from(entry.depth),
                    score,
                    bound: entry.bound_and_age & 3,
                    mv: entry.mv,
                });
            }
        }
        None
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        if !self.buckets.is_null() {
            // This base belongs to VirtualAlloc, not the Rust heap allocator.
            unsafe {
                VirtualFree(self.buckets.cast(), 0, MEM_RELEASE);
            }
        }
    }
}
