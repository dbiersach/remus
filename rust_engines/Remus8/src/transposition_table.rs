//! Remember searched positions in four-entry, 64-byte buckets.
//!
//! The table holds a configurable number of buckets with multiply-shift hashing. A full bucket replaces its
//! least useful entry according to search age and depth. All search threads share it.

use std::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};
use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering::Relaxed};

use crate::board::{MATE_SCORE, Move};

pub const EXACT: u8 = 0;
pub const LOWER: u8 = 1;
pub const UPPER: u8 = 2;

const MIB: usize = 1024 * 1024;
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
const PAYLOAD_MASK: u64 = (1_u64 << 58) - 1; // Age is independent of key validation
const AGE_WEIGHT: i32 = 8; // One search of age counts as much as this many plies

/// Each entry holds the key XOR the score/move/depth/bound payload, then data.
/// Age occupies the upper six data bits and is excluded from key validation, so
/// refreshing age with a single compare-exchange cannot mix score and key words.
/// Concurrent payload writers can produce a cache miss; readers validate the key.
#[repr(align(64))]
struct Bucket {
    slots: [[AtomicU64; 2]; ENTRIES_PER_BUCKET],
}

const _: () = assert!(size_of::<Bucket>() == 64);

/// Pack score, move, depth, bound, and age into one word.
#[inline(always)]
fn pack(score: i32, mv: Move, depth: u8, bound_and_age: u8) -> u64 {
    u64::from(score as u32)
        | (u64::from(mv) << 32)
        | (u64::from(depth) << 48)
        | (u64::from(bound_and_age) << 56)
}

/// Map the whole 64-bit key range onto the fixed table.
#[inline]
fn bucket_index(key: u64, bucket_count: usize) -> usize {
    ((u128::from(key) * bucket_count as u128) >> 64) as usize
}

/// Obtain aligned zero-filled pages without touching the whole table at startup.
fn allocate_buckets(bytes: usize) -> *mut Bucket {
    // Windows aligns the base to at least a page boundary, exceeding Bucket's 64
    // bytes. Zero bits are valid atomics. Pages are supplied on access.
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
#[derive(Clone, Copy)]
pub struct Stored {
    data: u64,
    pub depth: i32,
    pub score: i32,
    pub bound: u8,
    pub mv: Move,
}

#[derive(Default)]
pub struct StoreResult {
    pub replaced: bool,
    pub protected: bool,
}

pub struct Table {
    buckets: *mut Bucket,
    bucket_count: usize,
    age: AtomicU8,
}

// Every access to the buckets goes through atomic words, so threads may share them.
unsafe impl Send for Table {}
unsafe impl Sync for Table {}

impl Table {
    pub fn new() -> Table {
        Self::with_size(512)
    }

    pub fn disabled() -> Table {
        Table {
            buckets: ptr::null_mut(),
            bucket_count: 0,
            age: AtomicU8::new(0),
        }
    }

    pub fn with_size(mib: usize) -> Table {
        assert!(matches!(mib, 512 | 1024 | 2048));
        Table {
            buckets: allocate_buckets(mib * MIB),
            bucket_count: mib * MIB / size_of::<Bucket>(),
            age: AtomicU8::new(0),
        }
    }

    pub fn size_mib(&self) -> usize {
        if self.buckets.is_null() {
            0
        } else {
            self.bucket_count * size_of::<Bucket>() / MIB
        }
    }

    /// Estimate occupied slots, including older searches, from 4,096 spread buckets.
    /// Call between searches, after all workers have finished writing the table.
    pub fn fill_percentage(&self) -> f64 {
        if self.buckets.is_null() {
            return 0.0;
        }
        let sampled_buckets = self.bucket_count.min(4096);
        let stride = self.bucket_count / sampled_buckets;
        let mut occupied = 0;
        for sample in 0..sampled_buckets {
            let index = sample * stride + stride / 2;
            // Each midpoint is inside the live allocation, away from adjacent samples.
            let slots = unsafe { &(*self.buckets.add(index)).slots };
            for slot in slots {
                let data = slot[1].load(Relaxed);
                if slot[0].load(Relaxed) ^ (data & PAYLOAD_MASK) != 0 {
                    occupied += 1;
                }
            }
        }
        100.0 * occupied as f64 / (sampled_buckets * ENTRIES_PER_BUCKET) as f64
    }

    /// Mark the start of a new search, so older entries give way first.
    pub fn new_search(&self) {
        let age = self.age.load(Relaxed);
        self.age.store((age + 1) % AGE_LIMIT, Relaxed);
    }

    /// Save one search result. A mate score is stored as mate from here.
    #[inline]
    pub fn store(
        &self,
        key: u64,
        depth: i32,
        mut score: i32,
        bound: u8,
        mv: Move,
        ply: i32,
    ) -> StoreResult {
        if score > MATE_SCORE - 1000 {
            score += ply;
        } else if score < -MATE_SCORE + 1000 {
            score -= ply;
        }
        if self.buckets.is_null() || key == 0 {
            return StoreResult::default(); // Zero marks an empty entry, so this key is not cached.
        }
        let age = self.age.load(Relaxed);
        let index = bucket_index(key, self.bucket_count);
        // The index is inside the owned allocation, and every word is atomic.
        let slots = unsafe { &(*self.buckets.add(index)).slots };
        let mut chosen = 0;
        let mut replacement = true;
        let mut lowest_value = i32::MAX;
        for (i, slot) in slots.iter().enumerate() {
            let data = slot[1].load(Relaxed);
            let stored_key = slot[0].load(Relaxed) ^ (data & PAYLOAD_MASK);
            if stored_key == key {
                let stored_depth = i32::from((data >> 48) as u8);
                let stored_bound = (data >> 56) as u8 & 3;
                if stored_depth > depth
                    || (stored_depth == depth && stored_bound == EXACT && bound != EXACT)
                {
                    return StoreResult {
                        replaced: false,
                        protected: true,
                    };
                }
            }
            if stored_key == key || stored_key == 0 {
                replacement = false;
                chosen = i;
                break;
            }
            let stored_age = (data >> 58) as u8;
            let searches_ago = i32::from(age.wrapping_sub(stored_age) % AGE_LIMIT);
            let value = i32::from((data >> 48) as u8) - AGE_WEIGHT * searches_ago;
            if value < lowest_value {
                lowest_value = value;
                chosen = i;
            }
        }
        let data = pack(score, mv, depth as u8, bound | (age << 2));
        slots[chosen][0].store(key ^ (data & PAYLOAD_MASK), Relaxed);
        slots[chosen][1].store(data, Relaxed);
        StoreResult {
            replaced: replacement,
            protected: false,
        }
    }

    /// Refresh a useful cutoff once per search generation. A changed payload or key
    /// makes this best-effort update a no-op; score, depth, bound, and move stay intact.
    #[inline]
    pub fn refresh(&self, key: u64, stored: &Stored) -> bool {
        let age = u64::from(self.age.load(Relaxed));
        if self.buckets.is_null() || key == 0 || stored.data >> 58 == age {
            return false;
        }
        let index = bucket_index(key, self.bucket_count);
        // The bucket belongs to this live table; only atomic words are accessed.
        let slots = unsafe { &(*self.buckets.add(index)).slots };
        for slot in slots {
            if slot[0].load(Relaxed) ^ (stored.data & PAYLOAD_MASK) == key {
                return slot[1]
                    .compare_exchange(
                        stored.data,
                        (stored.data & PAYLOAD_MASK) | (age << 58),
                        Relaxed,
                        Relaxed,
                    )
                    .is_ok();
            }
        }
        false
    }

    /// Ask the processor to start loading the bucket of a position. The table is far
    /// larger than the processor's cache, so a bucket takes about as long to arrive
    /// from memory as a whole position takes to search; asking as soon as the key is
    /// known lets the search work during that wait.
    #[inline(always)]
    pub fn prefetch(&self, key: u64) {
        if !self.buckets.is_null() {
            let bucket = self
                .buckets
                .wrapping_add(bucket_index(key, self.bucket_count));
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
        let index = bucket_index(key, self.bucket_count);
        // The allocation stays live while self is borrowed, and index is in bounds.
        let slots = unsafe { &(*self.buckets.add(index)).slots };
        for slot in slots {
            let data = slot[1].load(Relaxed);
            if slot[0].load(Relaxed) ^ (data & PAYLOAD_MASK) == key {
                let mut score = data as u32 as i32;
                if score > MATE_SCORE - 1000 {
                    score -= ply;
                } else if score < -MATE_SCORE + 1000 {
                    score += ply;
                }
                return Some(Stored {
                    data,
                    depth: i32::from((data >> 48) as u8),
                    score,
                    bound: (data >> 56) as u8 & 3,
                    mv: (data >> 32) as Move,
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
