# uci.py: Talk to Lucas Chess

## The big picture

Lucas Chess draws the board and runs the clocks. Remus does the thinking. The two
programs talk by sending lines of text, in a language called UCI, the Universal Chess
Interface. `uci.py` reads Lucas Chess's commands and sends back Remus's answers.
Each Remus engine has its own `uci.py`, and `remus.bat` starts it.

## Words to know

- **UCI:** the text language between chess GUIs and chess engines. Stockfish speaks
  it too.
- **GUI:** graphical user interface, the window with the board you click on.
- **Standard input and output:** the lines a program reads in and prints out.
  Lucas Chess writes to Remus's standard input and reads Remus's standard output.

## UCI moves

A UCI move is only the square a piece leaves and the square it lands on, with no
piece letter: `e2e4`, `g1f3`, and `e1g1` for White castling on the king's side. A
promotion adds the new piece in a small letter: `e7e8q`. People write moves in SAN
(`e4`, `Nf3`, `O-O`, `e8=Q`), but a program reads UCI more easily, so Remus reads and
writes only UCI.

## A short conversation

```text
Lucas Chess -> uci                          Remus -> id name Remus2
                                            Remus -> uciok
Lucas Chess -> isready                      Remus -> readyok
Lucas Chess -> position startpos moves e2e4
Lucas Chess -> go wtime 60000 btime 60000   Remus -> info depth 1 score cp 10 nodes 43
                                            Remus -> ...
                                            Remus -> bestmove e7e5
Lucas Chess -> quit
```

## How it works

1. `main()` reads one line at a time and looks at its first word.
2. `uci` asks the engine's name. Remus answers `id name` and `uciok`.
3. `isready` asks whether Remus is ready. Remus answers `readyok`.
4. `position startpos moves e2e4 e7e5 ...` names the game so far. Remus starts from
   `START_POSITION` and plays every move after the word `moves`. Lucas Chess sends
   every game from the starting position, so Remus needs no FEN.
5. `go` starts the search. `read_time()` reads the clock (`wtime` or `btime`) and the
   increment (`winc` or `binc`) of the side to move. Remus plans to spend a thirtieth
   of its clock plus half its increment, or exactly `movetime` when Lucas sends one.
6. `think()` in `search.py` searches one move deeper at a time and prints an `info`
   line after each depth: the depth, the score in centipawns, and the positions
   visited. When the time is up, Remus sends `bestmove` and the move.
7. With no legal move (checkmate or stalemate), Remus sends `bestmove 0000`.
8. `quit` ends the program.

Remus prints only UCI lines, and the last line of each answer passes `flush=True` to
`print()`, so the answer reaches Lucas Chess right away instead of waiting in a
buffer.

## What Remus3 and Remus4 add

- **The opening book.** Remus3 and Remus4 call `load_book()` once at the start. On
  `go`, `legal_book_move()` looks for the game's moves at the start of a line of
  `opening_book.txt`, checks the next book move with the move generator, and plays it
  at once without searching.
- **A hard time limit.** Remus3 and Remus4 pass `think()` a planned time and a
  longest time: three times the plan, but never more than a fifth of the clock. The
  search takes longer than planned when its best move keeps changing.
- **The game history.** Remus4 keeps the position key of every position of the game
  in `game_history`, so its search can see a threefold repetition coming.

## A whole conversation to read

This section is extra reading, not a task of any period. Here is everything Remus2
says when it is asked to choose a first move with two seconds to think:

```text
Lucas Chess -> uci                          Remus -> id name Remus2
                                            Remus -> uciok
Lucas Chess -> isready                      Remus -> readyok
Lucas Chess -> position startpos
Lucas Chess -> go movetime 2000             Remus -> info depth 1 score cp 50 nodes 41
                                            Remus -> info depth 2 score cp 0 nodes 266
                                            Remus -> info depth 3 score cp 50 nodes 2505
                                            Remus -> info depth 4 score cp 0 nodes 17360
                                            Remus -> info depth 5 score cp 40 nodes 153457
                                            Remus -> bestmove g1f3
Lucas Chess -> quit
```

Each `info` line reports one finished depth. At depth 1 Remus2 tries each of White's
20 first moves, and the best of them scores 50 centipawns, half a pawn. At depth 2 it
also tries every Black reply, and the score falls to 0. The score swings up at the
odd depths, where White makes the last move, and down at the even ones, where Black
answers. Each depth visits six to nine times as many positions as the one before, so
depth 5 visits 153,457. Remus2 starts a new depth only while less than half of its
time is gone. After depth 5 more than a second has passed, so Remus2 sends its best
move from depth 5: `g1f3`, the knight to f3.

## Puzzle questions

1. Why must every line Remus prints be a UCI line, with nothing else mixed in?
2. What does Remus send when it has no legal move?
3. How does Lucas Chess write the move of a knight from g1 to f3?
