//! Remember searched positions in four-entry, 64-byte buckets.
//!
//! The table targets half the installed physical RAM, read once at startup.
//! Multiply-shift indexing permits any bucket count. A full bucket replaces its
//! least useful entry according to search age and depth.

use std::ffi::c_void;
use std::ptr;

use crate::board::{MATE_SCORE, Move};

pub const EXACT: u8 = 0;
pub const LOWER: u8 = 1;
pub const UPPER: u8 = 2;

const DEFAULT_BUCKETS: usize = 1 << 23; // 512 MiB if installed RAM is unknown
const MEM_COMMIT: u32 = 0x1000; // Back the region with Windows commit capacity
const MEM_RESERVE: u32 = 0x2000; // Reserve addresses for the table
const MEM_RELEASE: u32 = 0x8000; // Release the whole allocation on drop
const PAGE_READWRITE: u32 = 0x04; // Ordinary writable, nonexecutable pages

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetPhysicallyInstalledSystemMemory(kilobytes: *mut u64) -> i32;
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

/// Return installed RAM, independent of current free or in-use memory.
fn physical_memory_bytes() -> Option<u64> {
    let mut kilobytes = 0;
    // The pointer names a live u64 for the duration of this synchronous call.
    if unsafe { GetPhysicallyInstalledSystemMemory(&mut kilobytes) } == 0 || kilobytes == 0 {
        return None;
    }
    kilobytes.checked_mul(1024)
}

/// Count whole buckets within half the installed RAM and pointer arithmetic limits.
fn bucket_count(installed_bytes: Option<u64>) -> usize {
    match installed_bytes {
        Some(bytes) if bytes > 0 => {
            let count = bytes / 2 / size_of::<Bucket>() as u64;
            count.clamp(1, (isize::MAX as usize / size_of::<Bucket>()) as u64) as usize
        }
        _ => DEFAULT_BUCKETS,
    }
}

/// Map the whole 64-bit key range onto any nonzero number of buckets.
#[inline]
fn bucket_index(key: u64, count: usize) -> usize {
    ((u128::from(key) * count as u128) >> 64) as usize
}

/// Obtain aligned zero-filled pages without touching the whole table at startup.
fn allocate_buckets(count: usize) -> *mut Bucket {
    let Some(bytes) = count.checked_mul(size_of::<Bucket>()) else {
        return ptr::null_mut();
    };
    if bytes == 0 || bytes > isize::MAX as usize {
        return ptr::null_mut();
    }
    // Windows aligns the base to at least a page boundary, exceeding Bucket's 64
    // bytes. Integer-only entries accept zero bits. Pages are supplied on access.
    unsafe {
        VirtualAlloc(
            ptr::null_mut(),
            bytes,
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
    bucket_count: usize,
    age: u8,
}

impl Table {
    pub fn new() -> Table {
        Self::with_allocator(bucket_count(physical_memory_bytes()), allocate_buckets)
    }

    /// Retry smaller allocations on failure; zero buckets disables only caching.
    fn with_allocator(mut count: usize, allocate: fn(usize) -> *mut Bucket) -> Table {
        let mut buckets = ptr::null_mut();
        while count > 0 {
            buckets = allocate(count);
            if !buckets.is_null() {
                break;
            }
            count /= 2;
        }
        Table {
            buckets,
            bucket_count: count,
            age: 0,
        }
    }

    /// Mark the start of a new search, so older entries give way first.
    pub fn new_search(&mut self) {
        self.age = (self.age + 1) % AGE_LIMIT;
    }

    /// Save one search result. A mate score is stored as mate from here.
    #[inline]
    pub fn store(&mut self, key: u64, depth: i32, mut score: i32, bound: u8, mv: Move, ply: i32) {
        if score > MATE_SCORE - 1000 {
            score += ply;
        } else if score < -MATE_SCORE + 1000 {
            score -= ply;
        }
        if self.bucket_count == 0 || key == 0 {
            return; // Zero marks an empty entry, so this key is not cached.
        }
        let age = self.age;
        let index = bucket_index(key, self.bucket_count);
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
        entries[chosen] = Entry {
            key,
            score,
            mv,
            depth: depth as u8,
            bound_and_age: bound | (age << 2),
        };
    }

    /// Return what the table holds for the position, or None.
    #[inline]
    pub fn probe(&self, key: u64, ply: i32) -> Option<Stored> {
        if self.bucket_count == 0 || key == 0 {
            return None;
        }
        let index = bucket_index(key, self.bucket_count);
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
