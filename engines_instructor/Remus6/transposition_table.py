"""transposition_table.py

Remember what the search learned about each position: the depth it
searched, the score, whether that score is exact or only a bound, and
the best move. Two move orders often reach the same position, and the
second time the search can reuse the score or try the best move first.
"""

import ctypes

from board import MATE_SCORE, Move

EXACT, LOWER, UPPER = 0, 1, 2  # The score is exact, at least, or at most
ENTRY_BYTES = 260  # Estimated bytes per dictionary entry, not a hard byte limit
DEFAULT_MAX_ENTRIES = 2_000_000  # Used only if installed RAM cannot be read

table: dict[int, tuple[int, int, int, Move | None]] = {}


def physical_memory_bytes() -> int:
    """Return installed RAM in bytes, or 0 when Windows cannot report it."""
    try:
        query = ctypes.WinDLL("kernel32").GetPhysicallyInstalledSystemMemory
        query.argtypes = [ctypes.POINTER(ctypes.c_ulonglong)]
        query.restype = ctypes.c_int
        kilobytes = ctypes.c_ulonglong()
        if query(ctypes.byref(kilobytes)):
            return kilobytes.value * 1024
    except OSError:
        pass
    return 0


def entry_limit(installed_bytes: int) -> int:
    """Budget half the installed RAM using the estimated cost of one entry."""
    if installed_bytes <= 0:
        return DEFAULT_MAX_ENTRIES
    return max(1, installed_bytes // 2 // ENTRY_BYTES)


# Read installed RAM once at import; the dictionary allocates entries as it grows.
MAX_ENTRIES = entry_limit(physical_memory_bytes())


def store(key: int, depth: int, score: int, bound: int, move: Move | None, ply: int):
    """Save one search result. A mate score is stored as mate from here."""
    if score > MATE_SCORE - 1000:
        score += ply
    elif score < -MATE_SCORE + 1000:
        score -= ply
    if len(table) > MAX_ENTRIES:
        table.clear()
    try:
        table[key] = (depth, score, bound, move)
    except MemoryError:
        # Cached results are optional; release them if a table allocation fails.
        table.clear()


def probe(key: int, ply: int) -> tuple[int, int, int, Move | None] | None:
    """Return (depth, score, bound, move) for the position, or None."""
    entry = table.get(key)
    if entry is None:
        return None
    depth, score, bound, move = entry
    if score > MATE_SCORE - 1000:
        score -= ply
    elif score < -MATE_SCORE + 1000:
        score += ply
    return depth, score, bound, move
