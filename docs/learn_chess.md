# Learn chess: the rules that trip people up, and how moves are written

You know how the pieces move. This page holds the rest: the rules that catch nearly
everyone, the value of each piece, and the way people and programs write a move down.
Each rule names the engine whose code needs it, so you can read the rule right before
you type that code.

## The rules that trip people up

### Check, pins, and discovered check (Remus1)

A move is legal only if your own king is safe after it. That one rule covers three
cases. You may not leave your king in check. You may not move a **pinned** piece off
the line between an enemy piece and your king, because the king would then be in
check. You may not make a move that uncovers an enemy piece's line to your king, a
**discovered check** against yourself. Remus checks all three the same way: it makes
the move on a copy of the board and asks whether its own king is attacked there.

![A Black rook on e8 checks the White king on e1](../images/chess_tutor/pin_check.svg) ![A White knight on e2 blocks the check, so it is pinned](../images/chess_tutor/pin_knight.svg)

*On the left, White is in check and must deal with the rook. On the right, the knight
on e2 blocks the check, so it is pinned: any knight move would leave the king in
check, and none of them is legal.*

### Castling (Remus1)

The king moves two squares toward a rook, and that rook jumps over the king to the
square beside it. Castling is allowed only when all of these hold:

1. The king and that rook have never moved.
2. The squares between them are empty.
3. The king is not in check.
4. The king does not pass through, or land on, a square that an enemy piece attacks.

The rook may cross an attacked square: on the queen's side, an attack on b1 does not
stop White from castling. A rook that moves loses its castling right for good, even
if it comes back, and a rook captured on its home square loses it too. Remus keeps
the four rights as four on-off switches in one number, and turns a switch off when a
king or rook leaves its home square or a rook is captured there.

![The six home squares of the kings and rooks](../images/chess_tutor/castling_rights.svg)

*The marked squares are the six home squares that hold the castling rights. A move
from one of them, or a capture on one of them, turns off a castling right.*

### En passant (Remus1)

When a pawn moves two squares and lands beside an enemy pawn, that enemy pawn may
capture it on the very next move only, as if it had moved one square. The capturing
pawn lands on the square the pawn skipped, and the captured pawn disappears from the
square beside it, not from the square the capturer lands on. The rare trap: an en
passant capture can open a line along the rank to your own king, so Remus tests the
king's safety after the captured pawn is gone.

![White captures en passant from e5 to d6, removing the pawn on d5](../images/chess_tutor/en_passant.svg)

*Black has just played d7 to d5. White's pawn on e5 captures on d6, and the pawn on d5
leaves the board.*

### Promotion and underpromotion (Remus1)

A pawn that reaches the last rank becomes a queen, a rook, a bishop, or a knight,
the player's choice. Choosing anything but a queen is an **underpromotion**. It is
rare, but sometimes it is the only move that wins: a knight can promote with check
and a fork, and a new queen can give stalemate by mistake. One side may end up with
two queens, so Remus never assumes one queen per side.

![White king c6, bishop b6, and pawn b7 against the Black king on a6](../images/chess_tutor/promotion_puzzle.svg)

*White to move. A new queen on b8 attacks every square the Black king could go to but
does not give check, so Black has no legal move and the game is a draw by stalemate. A
new knight on b8 gives check, and the king still has no square: checkmate.*

### The draws (Remus2 and Remus4)

| Draw | The rule |
| --- | --- |
| Stalemate | The side to move has no legal move and is not in check. The game is a draw, not a loss |
| Threefold repetition | The same position, with the same side to move and the same castling and en passant rights, appears three times. The three times need not be in a row. A player may claim the draw; at five times the game ends by itself |
| Fifty-move rule | Fifty moves by each side (100 half-moves) with no capture and no pawn move. A player may claim the draw at 50; at 75 the game ends by itself |
| Insufficient material | Neither side can ever give checkmate: king against king, king and bishop against king, king and knight against king, and king and bishop against king and bishop with both bishops on the same color of square |

Remus2's search already scores stalemate as a draw. Remus4 adds the other three: it
counts half-moves toward the fifty-move rule, keeps a `history` list of the positions
in the game to spot a repeat, and scores a board with no pawns and at most one
knight or bishop as a draw.

### Piece values and the three tactics (Remus2)

| Piece | Value in pawns |
| --- | ---: |
| Pawn | 1 |
| Knight | 3 |
| Bishop | 3 |
| Rook | 5 |
| Queen | 9 |

The king has no value because it can never be captured. Remus scores in
**centipawns**, hundredths of a pawn, so a knight is 300 in `PIECE_VALUES` in
`evaluation.py`. Three tactics win material with these values: a **fork** attacks two
pieces at once, a **pin** holds a piece in front of a more valuable one, and a
**skewer** attacks a valuable piece so that the piece behind it falls when it moves.

## Chess notation: how people write down moves

Chess players write every move of a game so they can play it over later, send it to a
friend, or study it in a book. The usual way is called **SAN**, short for Standard
Algebraic Notation. Lucas Chess lists every move of a game in SAN.

### Name every square

The board has eight **files**, the columns `a` to `h` from White's left to White's
right, and eight **ranks**, the rows `1` to `8` from White's side to Black's side.
A square's name is its file letter, then its rank number.

- White's king starts on `e1`, and Black's king starts on `e8`.
- The corner to White's left is `a1`. It is always a dark square.
- The square in front of White's king pawn is `e3`, and two squares ahead is `e4`.

### Name every piece

Each piece except the pawn has a capital letter:

| Piece | Letter |
| --- | --- |
| King | `K` |
| Queen | `Q` |
| Rook | `R` |
| Bishop | `B` |
| Knight | `N` (because `K` is the king) |
| Pawn | no letter |

### Write a move

A move is the piece letter, then the square it lands on.

- `Nf3`: a knight moves to f3.
- `Bc4`: a bishop moves to c4.
- `e4`: a pawn moves to e4. A pawn move is only the square.

### Captures

An `x` means the piece takes something on the square.

- `Bxf7`: a bishop captures on f7.
- `exd5`: a pawn captures on d5. A pawn capture starts with the file the pawn came
  from, here the e-file, because the pawn moves diagonally.

### Two pieces that could make the same move

When two of the same piece can reach a square, the move adds the file or the rank
the piece leaves from, so the reader knows which one moved.

- Knights on b1 and f3 can both reach d2. `Nbd2` moves the knight from the b-file.
- Rooks on a1 and a5 can both reach a3. `R1a3` moves the rook from rank 1.

### Castling, promotion, check, and checkmate

| Writing | Meaning |
| --- | --- |
| `O-O` | Castle on the king's side (the king moves two squares toward the h-file) |
| `O-O-O` | Castle on the queen's side (the king moves two squares toward the a-file) |
| `e8=Q` | A pawn reaches the last rank and becomes a queen |
| `e8=N` | The same pawn becomes a knight instead (an underpromotion) |
| `+` at the end | The move gives check, such as `Bb5+` |
| `#` at the end | The move gives checkmate, such as `Qxf7#` |

Castling uses the capital letter O, not the number zero.

### Read a whole move

```text
N  b  x  d2  +
|  |  |  |   gives check
|  |  |  square it lands on
|  |  captures
|  which knight (the one on the b-file)
piece letter (no letter for a pawn)
```

### A short game

A game is written as numbered moves, White's move first, then Black's.

```text
1. e4 e5 2. Bc4 Nc6 3. Qh5 Nf6 4. Qxf7#
```

On move 3 Black's knight on f6 attacks the queen, but only the king guards the pawn
on f7. On move 4 the queen captures that pawn with checkmate: the bishop on c4
protects the queen, so the king cannot take it, and every square the king could step
to is attacked or blocked.

### The notation chess programs use

Lucas Chess and Remus talk to each other in **UCI** notation instead. A UCI move is
only the square a piece leaves and the square it lands on, with no piece letter:
`e2e4`, `g1f3`, and `e1g1` for White castling on the king's side. A promotion adds
the new piece in a small letter: `e7e8q`. UCI is easy for a program to read but
harder for people, so Remus reads and writes only UCI, and Lucas Chess turns each
move into SAN for the person watching. [`uci.md`](uci.md) shows the whole
conversation between the two programs.

### Questions

1. What is the SAN of a pawn on g7 that captures on h8 and becomes a queen?
2. Rooks on a1 and h1 can both move to d1. How do you write each move?
3. The knight on g1 moves to f3. How does Lucas Chess write that move in UCI?
