"""transposition_table.py

Remember search results in a fixed allocation of four-entry buckets.
A full bucket replaces an entry according to its search age and depth.
"""

import mmap
import struct

from board import MATE_SCORE, Move

EXACT, LOWER, UPPER = 0, 1, 2  # The score is exact, at least, or at most
TABLE_BYTES = 2_147_483_648  # Fixed 2 GiB cache
ENTRY = struct.Struct("<QiHBB")  # Key, score, packed move, depth, bound and age
ENTRIES_PER_BUCKET = 4  # Four 16-byte entries occupy one 64-byte bucket
BUCKET_BYTES = ENTRY.size * ENTRIES_PER_BUCKET
BUCKET_COUNT = TABLE_BYTES // BUCKET_BYTES
KEY_MASK = (1 << 64) - 1  # Store Python's signed position hash as an unsigned key
AGE_LIMIT = 64  # Ages wrap around at this count of searches
AGE_WEIGHT = 8  # One search of age counts as much as this many plies


def allocate_table() -> mmap.mmap | None:
    """Allocate zero-filled pages; an unavailable allocation disables caching."""
    try:
        return mmap.mmap(-1, TABLE_BYTES)
    except OSError, MemoryError:
        return None


table = allocate_table()
age = 0  # The current search's generation within AGE_LIMIT


def new_search() -> None:
    """Mark a new search so older entries give way first."""
    global age
    age = (age + 1) % AGE_LIMIT


def store(key: int, depth: int, score: int, bound: int, move: Move | None, ply: int):
    """Save one search result. A mate score is stored as mate from here."""
    key &= KEY_MASK
    if table is None or key == 0:
        return
    if score > MATE_SCORE - 1000:
        score += ply
    elif score < -MATE_SCORE + 1000:
        score -= ply
    offset = ((key * BUCKET_COUNT) >> 64) * BUCKET_BYTES
    chosen = offset
    lowest_value = 1_000_000  # Above every entry's depth minus its age penalty
    for entry_offset in range(offset, offset + BUCKET_BYTES, ENTRY.size):
        stored_key, _, _, stored_depth, bound_and_age = ENTRY.unpack_from(
            table, entry_offset
        )
        if stored_key == key or stored_key == 0:
            chosen = entry_offset
            break
        searches_ago = (age - (bound_and_age >> 2)) % AGE_LIMIT
        value = stored_depth - AGE_WEIGHT * searches_ago
        if value < lowest_value:
            lowest_value = value
            chosen = entry_offset
    packed_move = 0
    if move is not None:
        packed_move = (move[0] << 9) | (move[1] << 3) | move[2]
    ENTRY.pack_into(table, chosen, key, score, packed_move, depth, bound | (age << 2))


def probe(key: int, ply: int) -> tuple[int, int, int, Move | None] | None:
    """Return (depth, score, bound, move) for the position, or None."""
    key &= KEY_MASK
    if table is None or key == 0:
        return None
    offset = ((key * BUCKET_COUNT) >> 64) * BUCKET_BYTES
    for entry_offset in range(offset, offset + BUCKET_BYTES, ENTRY.size):
        stored_key, score, packed_move, depth, bound_and_age = ENTRY.unpack_from(
            table, entry_offset
        )
        if stored_key == key:
            if score > MATE_SCORE - 1000:
                score -= ply
            elif score < -MATE_SCORE + 1000:
                score += ply
            move = None
            if packed_move:
                move = (packed_move >> 9, (packed_move >> 3) & 63, packed_move & 7)
            return depth, score, bound_and_age & 3, move
    return None
