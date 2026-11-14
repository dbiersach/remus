from board import MATE_SCORE

EXACT, LOWER, UPPER = 0, 1, 2
MAX_ENTRIES = 2_000_000

table = {}


def store(key, depth, score, bound, move, ply):
    if score > MATE_SCORE - 1000:
        score += ply
    elif score < -MATE_SCORE + 1000:
        score -= ply
    if len(table) > MAX_ENTRIES:
        table.clear()
    table[key] = (depth, score, bound, move)


def probe(key, ply):
    entry = table.get(key)
    if entry is None:
        return None
    depth, score, bound, move = entry
    if score > MATE_SCORE - 1000:
        score -= ply
    elif score < -MATE_SCORE + 1000:
        score += ply
    return depth, score, bound, move
