//! Remember what the search learned about each position.
//!
//! The table has a fixed 512 MiB capacity. Each bucket is one
//! 64-byte cache line, so a look at all four entries costs one trip to memory. A position
//! can live in any of its bucket's four entries. A full bucket gives up its least useful
//! entry: the one left by the oldest search, then the shallowest one. Until a bucket
//! fills, the table behaves exactly like the Python dict of Remus4.

use std::alloc::{Layout, alloc_zeroed};

use crate::board::{MATE_SCORE, Move};

pub const EXACT: u8 = 0;
pub const LOWER: u8 = 1;
pub const UPPER: u8 = 2;

const MIB: usize = 1024 * 1024;
const TABLE_BYTES: usize = 512 * MIB;
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

/// What probe() found: the depth, score, bound, and best move of a position.
pub struct Stored {
    pub depth: i32,
    pub score: i32,
    pub bound: u8,
    pub mv: Move,
}

pub struct Table {
    buckets: Box<[Bucket]>,
    age: u8,
}

impl Table {
    pub fn new() -> Table {
        let count = TABLE_BYTES / size_of::<Bucket>();
        let layout = Layout::array::<Bucket>(count).expect("the table size fits in memory");
        let buckets = unsafe {
            let memory = alloc_zeroed(layout);
            if memory.is_null() {
                return Self::disabled();
            }
            let slice = std::ptr::slice_from_raw_parts_mut(memory.cast::<Bucket>(), count);
            Box::from_raw(slice)
        };
        Table { buckets, age: 0 }
    }

    pub fn disabled() -> Table {
        Table {
            buckets: Box::new([]),
            age: 0,
        }
    }

    pub fn size_mib(&self) -> usize {
        self.buckets.len() * size_of::<Bucket>() / MIB
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
        if self.buckets.is_empty() {
            return false;
        }
        if score > MATE_SCORE - 1000 {
            score += ply;
        } else if score < -MATE_SCORE + 1000 {
            score -= ply;
        }
        let age = self.age;
        let index = key as usize & (self.buckets.len() - 1);
        let entries = &mut self.buckets[index].entries;
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

    /// Return what the table holds for the position, or None.
    #[inline]
    pub fn probe(&self, key: u64, ply: i32) -> Option<Stored> {
        if self.buckets.is_empty() {
            return None;
        }
        let entries = &self.buckets[key as usize & (self.buckets.len() - 1)].entries;
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
