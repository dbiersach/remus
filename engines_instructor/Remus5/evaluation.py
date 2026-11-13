"""evaluation.py

Score a position in centipawns (a pawn is 100) for the side to move.
White's score adds up material, piece-square tables, mobility, the
bishop pair, rooks on open files, pawn structure, the pawn race, king
safety, and a mating bonus; Black's score is subtracted. The tables show
White's view with rank 8 on top, and a Black piece reads them upside
down. The king's table tapers from its middlegame table to its endgame
table as the pieces come off. Backward pawns, central holes, and stable
knight outposts add positional terms.
"""

from attacks import DIAGONAL, DIRECTIONS, KNIGHT_JUMPS, STRAIGHT
from board import BISHOP, EMPTY, KING, KNIGHT, PAWN, QUEEN, ROOK, WHITE, Position
from move_generation import piece_moves


def mirrored_table(left_half: tuple[int, ...]) -> tuple[int, ...]:
    """
    Return a 64-square table built from files a to d of every rank.

    The tables are the same on both sides of the board: a knight earns
    the same bonus on c3 and on f3. So each table lists only files a to
    d, rank 8 first, and files e to h repeat them in reverse order:
    file e copies d, f copies c, g copies b, and h copies a.
    """
    table = []
    for row in range(8):
        # Columns a, b, c, d, then d, c, b, a again for files e to h
        for col in (0, 1, 2, 3, 3, 2, 1, 0):
            table.append(left_half[row * 4 + col])
    return tuple(table)


# Pawns gain as they advance and hold the center
PAWN_TABLE = mirrored_table((
      0,   0,   0,   0,
     50,  50,  50,  50,
     10,  10,  20,  30,
      5,   5,  10,  25,
      0,   0,   0,  20,
      5,  -5, -10,   0,
      5,  10,  10, -20,
      0,   0,   0,   0,
))  # fmt: skip

# Knights are strongest in the center
KNIGHT_TABLE = mirrored_table((
    -50, -40, -30, -30,
    -40, -20,   0,   0,
    -30,   0,  10,  15,
    -30,   5,  15,  20,
    -30,   0,  15,  20,
    -30,   5,  10,  15,
    -40, -20,   0,   5,
    -50, -40, -30, -30,
))  # fmt: skip

# Bishops avoid the corners and edges
BISHOP_TABLE = mirrored_table((
    -20, -10, -10, -10,
    -10,   0,   0,   0,
    -10,   0,   5,  10,
    -10,   5,   5,  10,
    -10,   0,  10,  10,
    -10,  10,  10,  10,
    -10,   5,   0,   0,
    -20, -10, -10, -10,
))  # fmt: skip

# In the middlegame the king hides behind its pawns, near a corner
KING_MIDDLE_TABLE = mirrored_table((
    -30, -40, -40, -50,
    -30, -40, -40, -50,
    -30, -40, -40, -50,
    -30, -40, -40, -50,
    -20, -30, -30, -40,
    -10, -20, -20, -20,
     20,  20,   0,   0,
     20,  30,  10,   0,
))  # fmt: skip

# Rooks like the seventh rank and the central files
ROOK_TABLE = mirrored_table((
      0,   0,   0,   0,
      5,  10,  10,  10,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
     -5,   0,   0,   0,
      0,   0,   0,   5,
))  # fmt: skip

# In the endgame the king comes to the center
KING_END_TABLE = mirrored_table((
    -50, -40, -30, -20,
    -30, -20, -10,   0,
    -30, -10,  20,  30,
    -30, -10,  30,  40,
    -30, -10,  30,  40,
    -30, -10,  20,  30,
    -30, -30,   0,   0,
    -50, -30, -30, -30,
))  # fmt: skip

# What each kind of piece is worth, from empty (0) to king (6). The king is never
# captured, so it counts 0.
PIECE_VALUES = (0, 100, 300, 300, 500, 900, 0)

# Queens score the same on every square, and the king's tables taper below
ZERO_TABLE = (0,) * 64

# The table of each kind of piece, from empty (0) to king (6)
TABLES = (
    ZERO_TABLE, PAWN_TABLE, KNIGHT_TABLE, BISHOP_TABLE,
    ROOK_TABLE, ZERO_TABLE, ZERO_TABLE,
)  # fmt: skip

# The steps of each piece that mobility counts, and whether it slides
MOBILITY_STEPS = {
    KNIGHT: (KNIGHT_JUMPS, False), BISHOP: (DIAGONAL, True),
    ROOK: (STRAIGHT, True), QUEEN: (DIRECTIONS, True),
}  # fmt: skip

# The game phase: 24 with every piece on the board, 0 with none left
PHASE_VALUES = (0, 0, 1, 1, 2, 4, 0)  # By kind, from empty (0) to king (6)
# The bonus of a passed pawn by the moves it needs to promote
PASSED_BONUS = (0, 120, 80, 50, 30, 15, 10)
MOBILITY_BONUS = 2  # Per square a knight, bishop, rook, or queen reaches
BISHOP_PAIR = 30
ROOK_OPEN_FILE = 20  # A file with no pawns
ROOK_HALF_OPEN_FILE = 10  # A file with only enemy pawns
DOUBLED_PAWN = 15  # For each extra pawn on a file
ISOLATED_PAWN = 15  # No friendly pawn on a file beside it
BACKWARD_PAWN = 18  # An unsupported pawn whose advance meets an enemy pawn attack
BACKWARD_OPEN_FILE = 8  # Extra penalty when no enemy pawn screens its file
CENTRAL_HOLE = 12  # An empty central square an enemy knight can use
KNIGHT_OUTPOST = 15  # An advanced knight that enemy pawns cannot challenge
SUPPORTED_OUTPOST = 15  # Extra bonus when a friendly pawn protects the knight
UNSTOPPABLE_PAWN = 500  # A passed pawn the enemy king cannot catch
SHIELD_PAWN = 10  # For each pawn in front of a king on its back rank
OPEN_KING_FILE = 15  # For each file at the king with no friendly pawn


def pawn_attacks(pawns: tuple[list[int], list[int]]) -> tuple[list[bool], list[bool]]:
    """Mark pawn-controlled squares, including squares occupied by friendly pieces."""
    attacks = ([False] * 64, [False] * 64)
    for side in (0, 1):
        step = -1 if side == 0 else 1  # Row 0 is rank 8
        for square in pawns[side]:
            row, col = divmod(square, 8)
            for target_col in (col - 1, col + 1):
                if 0 <= row + step < 8 and 0 <= target_col < 8:
                    attacks[side][(row + step) * 8 + target_col] = True
    return attacks


def pawn_can_challenge(pawns: list[int], side: int, square: int) -> bool:
    """Can an adjacent-file pawn attack this square, directly or after advancing?"""
    row, col = divmod(square, 8)
    for pawn in pawns:
        behind = pawn // 8 > row if side == 0 else pawn // 8 < row
        if behind and abs(pawn % 8 - col) == 1:
            return True
    return False


def positional_terms(
    board: list[int],
    pawns: tuple[list[int], list[int]],
    knights: tuple[list[int], list[int]],
    attacks: tuple[list[bool], list[bool]],
) -> int:
    """Score backward pawns, central holes, and knight outposts from White's side."""
    score = 0
    for side in (0, 1):
        enemy = 1 - side
        sign = 1 if side == 0 else -1
        step = -8 if side == 0 else 8
        for square in pawns[side]:
            target = square + step
            if not 0 <= target < 64 or not attacks[enemy][target]:
                continue
            if attacks[side][square]:
                continue
            adjacent = False
            support = False
            for pawn in pawns[side]:
                if abs(pawn % 8 - square % 8) == 1:
                    adjacent = True
                    behind = (
                        pawn // 8 >= square // 8
                        if side == 0
                        else pawn // 8 <= square // 8
                    )
                    if behind:
                        support = True
            # Isolated pawns have their own penalty. Support can come from an
            # adjacent-file pawn on this rank or behind it.
            if adjacent and not support:
                penalty = BACKWARD_PAWN
                screened = False
                for pawn in pawns[enemy]:
                    if pawn % 8 == square % 8:
                        screened = True
                if not screened:
                    penalty += BACKWARD_OPEN_FILE
                score -= sign * penalty
        if knights[enemy]:
            # White's third and fourth ranks, mirrored for Black. An enemy pawn
            # protects the hole, and no friendly pawn can challenge its occupant.
            for view_row in (4, 5):
                row = view_row if side == 0 else 7 - view_row
                for col in range(2, 6):
                    square = row * 8 + col
                    is_hole = board[square] == EMPTY and attacks[enemy][square]
                    if is_hole and not pawn_can_challenge(pawns[side], side, square):
                        score -= sign * CENTRAL_HOLE
        for square in knights[side]:
            view_row = square // 8 if side == 0 else 7 - square // 8
            advanced = 2 <= view_row <= 4 and 1 <= square % 8 <= 6
            if advanced and not pawn_can_challenge(pawns[enemy], enemy, square):
                bonus = KNIGHT_OUTPOST
                if attacks[side][square]:
                    bonus += SUPPORTED_OUTPOST
                score += sign * bonus
    return score


def king_safety(
    pawns: tuple[list[int], list[int]],
    counts: tuple[list[int], list[int]],
    king: int,
    side: int,
    phase: int,
) -> int:
    """Score nearby pawn cover, tapering its value as pieces leave the board."""
    row, col = divmod(king, 8)
    view_row = row if side == 0 else 7 - row
    if view_row < 5:
        return 0  # An advanced king uses its piece-square table
    forward = -8 if side == 0 else 8
    score = 0
    for c in range(max(col - 1, 0), min(col + 2, 8)):
        if king + forward + c - col in pawns[side]:
            score += SHIELD_PAWN
        elif king + 2 * forward + c - col in pawns[side]:
            score += SHIELD_PAWN // 2
        if counts[side][c + 1] == 0:
            score -= OPEN_KING_FILE
    return score * phase // 24


def pawn_terms(pawns, rooks, kings, phase, white_to_move) -> int:
    """
    Score the pawns, the rooks that use their files, and the pawn shield
    of each king, from White's side.

    pawns and rooks hold the squares of White's pieces, then Black's.
    """
    score = 0
    # The pawns on each file, for each side. Entry col + 1 counts file col,
    # so the a and h files have an empty file on each side.
    counts = ([0] * 10, [0] * 10)
    for side in (0, 1):
        for square in pawns[side]:
            counts[side][square % 8 + 1] += 1
    for side in (0, 1):
        sign = 1 if side == 0 else -1
        enemy = 1 - side
        for count in counts[side]:
            score -= sign * DOUBLED_PAWN * max(count - 1, 0)
        for square in pawns[side]:
            row = square // 8
            col = square % 8
            if counts[side][col] == 0 and counts[side][col + 2] == 0:
                score -= sign * ISOLATED_PAWN
            # An enemy pawn ahead on this file or a file beside it can
            # stop the pawn
            passed = True
            for other in pawns[enemy]:
                ahead = other // 8 < row if side == 0 else other // 8 > row
                if ahead and abs(other % 8 - col) <= 1:
                    passed = False
            if not passed:
                continue
            steps = row if side == 0 else 7 - row  # Moves to promotion
            score += sign * PASSED_BONUS[steps]
            # The pawn race: with no pieces left, a pawn whose promotion
            # square the enemy king cannot reach (the rule of the square)
            # becomes a queen. A first move may go two squares.
            king = kings[enemy]
            king_steps = max(abs(king // 8 - 7 * side), abs(king % 8 - col))
            if white_to_move != (side == 0):
                king_steps -= 1  # The enemy king moves first
            if phase == 0 and king_steps > min(steps, 5):
                score += sign * UNSTOPPABLE_PAWN
        for square in rooks[side]:
            if counts[side][square % 8 + 1] == 0:
                open_file = counts[enemy][square % 8 + 1] == 0
                score += sign * (ROOK_OPEN_FILE if open_file else ROOK_HALF_OPEN_FILE)
        score += sign * king_safety(pawns, counts, kings[side], side, phase)
    return score


def mating_bonus(material: int, kings: list[int], phase: int) -> int:
    """
    In an endgame where one side is ahead, reward pushing the losing king
    to the edge and bringing the winning king close, from White's side.
    """
    if phase > 8 or abs(material) < 400:
        return 0
    winner = 0 if material > 0 else 1
    loser_row = kings[1 - winner] // 8
    loser_col = kings[1 - winner] % 8
    # 0 in the center, 6 in a corner
    edge = max(3 - loser_row, loser_row - 4) + max(3 - loser_col, loser_col - 4)
    distance = max(abs(kings[0] // 8 - kings[1] // 8), abs(kings[0] % 8 - kings[1] % 8))
    bonus = 10 * edge + 5 * (7 - distance)
    return bonus if winner == 0 else -bonus


def evaluate(position: Position) -> int:
    """Return the score for the side to move: positive when it is ahead."""
    board, white = position[0], position[1]
    score = 0  # From White's side
    material = 0
    phase = 0
    kings = [0, 0]  # The squares of the White king and the Black king
    pawns = ([], [])  # The squares of White's pawns, then Black's
    rooks = ([], [])
    knights = ([], [])
    for square in range(64):
        piece = board[square]
        if piece == EMPTY:
            continue
        kind = piece & 7
        side = 0 if piece & WHITE else 1
        sign = 1 if side == 0 else -1
        # Flip the row for Black: Black's a8 is White's a1
        view = square if side == 0 else (7 - square // 8) * 8 + square % 8
        material += sign * PIECE_VALUES[kind]
        score += sign * TABLES[kind][view]
        if kind == KING:
            kings[side] = square
        elif kind == PAWN:
            pawns[side].append(square)
        else:
            phase += PHASE_VALUES[kind]
            # Mobility: the moves the piece has, counted
            moves = []
            steps, slides = MOBILITY_STEPS[kind]
            piece_moves(board, side == 0, square, steps, slides, moves)
            score += sign * MOBILITY_BONUS * len(moves)
            if kind == ROOK:
                rooks[side].append(square)
            elif kind == KNIGHT:
                knights[side].append(square)
    # Insufficient material: a lone knight or bishop cannot mate
    if phase <= 1 and not pawns[0] and not pawns[1]:
        return 0
    phase = min(phase, 24)
    for side in (0, 1):
        sign = 1 if side == 0 else -1
        king = kings[side]
        view = king if side == 0 else (7 - king // 8) * 8 + king % 8
        taper = KING_MIDDLE_TABLE[view] * phase + KING_END_TABLE[view] * (24 - phase)
        score += sign * taper // 24
    if board.count(BISHOP + WHITE) >= 2:
        score += BISHOP_PAIR
    if board.count(BISHOP) >= 2:
        score -= BISHOP_PAIR
    score += material + pawn_terms(pawns, rooks, kings, phase, white)
    score += mating_bonus(material, kings, phase)
    score += positional_terms(board, pawns, knights, pawn_attacks(pawns))
    return score if white else -score
