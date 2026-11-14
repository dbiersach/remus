"""transposition_table.py

Remember what the search learned about each position: the depth it
searched, the score, whether that score is exact or only a bound, and
the best move. Two move orders often reach the same position, and the
second time the search can reuse the score or try the best move first.
"""

from board import MATE_SCORE, Move

EXACT, LOWER, UPPER = 0, 1, 2  # The score is exact, at least, or at most
MAX_ENTRIES = 2_000_000  # Start over when the table grows past this

table: dict[int, tuple[int, int, int, Move | None]] = {}


def store(key: int, depth: int, score: int, bound: int, move: Move | None, ply: int):
    """Save one search result. A mate score is stored as mate from here."""
    if score > MATE_SCORE - 1000:
        score += ply
    elif score < -MATE_SCORE + 1000:
        score -= ply
    if len(table) > MAX_ENTRIES:
        table.clear()
    table[key] = (depth, score, bound, move)


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
