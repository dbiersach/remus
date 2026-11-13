# FIDE titles and typical Elo ratings

The Elo rating carries the name of Arpad Elo (1903 to 1992), a Hungarian-American
physics professor and chess master who designed it. Elo is his surname, not an
acronym, so it is written "Elo", not "ELO". US Chess adopted his system in 1960 and
FIDE in 1970. An Elo rating predicts results: two players with the same rating each
expect to score half the points between them, a player 200 points higher expects to
score about 76%, and a player 400 points higher about 91%. After each game, a rating
rises when the player scores more than that prediction and falls when the player
scores less, so beating a stronger opponent gains more points than beating a weaker
one. The scale has no maximum. The highest rating any person has held is 2882, set by
Magnus Carlsen in 2014, and the strongest chess engines rate above 3500 on engine
rating lists.

FIDE, the international chess federation, awards titles to players who reach a
minimum Elo rating. The class names below
the titles (Expert and Class A to Class D) come from national federations such as
US Chess, not from FIDE.

| Title | Minimum rating requirement | Typical rating range |
| --- | ---: | ---: |
| **Super Grandmaster** (unofficial) | 2700+ | 2700 to 2850 |
| **Grandmaster (GM)** | 2500 | 2500 to 2650 |
| **International Master (IM)** | 2400 | 2400 to 2500 |
| **FIDE Master (FM)** | 2300 | 2300 to 2400 |
| **Candidate Master (CM)** | 2200 | 2200 to 2300 |
| Expert (not a FIDE title) | None | 2000 to 2199 |
| Class A player | None | 1800 to 1999 |
| Class B player | None | 1600 to 1799 |
| Class C player | None | 1400 to 1599 |
| Class D player | None | 1200 to 1399 |

## Ratings of everyday players

Most chess players never earn a title. The table below places everyday players on the
same Elo scale, from someone who has just learned the moves to a FIDE Master. The
ranges are estimates: club ratings differ from one federation to another, and many
casual players have no rating at all.

| Player category | Typical Elo range | Representative rating |
| --- | ---: | ---: |
| Casual player who knows the rules | 400 to 900 | 650 |
| Beginning chess club member | 600 to 1200 | 900 |
| Strong chess club player | 1200 to 1700 | 1450 |
| First-board player on a club team | 1500 to 2000 | 1750 |
| Average city chess club member | 1200 to 1600 | 1400 |
| Strong city chess club competitor | 1600 to 2000 | 1800 |
| City club champion | 1800 to 2200+ | 2000 |
| FIDE Candidate Master | 2200+ | 2250 |
| FIDE Master | 2300+ | 2350 |

## Where the Remus engines would land

FIDE gives titles only to people, so no engine can earn one. The figures below
are estimates from the benchmark described here, not human FIDE ratings or expected
results for a workshop test. The class labels are numerical comparisons only;
sharing a number does not establish equivalent strength against people. Remus2, Remus3,
Remus4, and Remus4-Rust each played 200 games against Stockfish held to a set
strength (its UCI_Elo setting), at 15 minutes per side with no increment, half with
White and half with Black, on one fast processor core per game. Each engine met
Stockfish at six levels between 1320 and 2950, and its rating is the one that best
explains its score against those levels. The 27 games that Stockfish lost on time
are left out, because Stockfish ran out of time in long endgames whatever the
position on the board.

| Engine | Benchmark Elo estimate | Reported 95% interval | Numerical class comparison only |
| --- | ---: | ---: | --- |
| Remus1 | 300 (estimate) | | None: a random mover sits far below Class D |
| Remus2 | 1840 | 1785 to 1895 | Class A player |
| Remus3 | 2086 | 2029 to 2143 | Expert |
| Remus4 | 2195 | 2133 to 2257 | Expert, at the edge of Candidate Master (CM) |
| Remus4-Rust | 2897 | 2831 to 2963 | Super Grandmaster |

Remus1's 300 figure is a rough estimate, not a fitted result in this table. Random
move choice does not guarantee a loss in every game.

A 95% confidence interval describes a procedure that, under its statistical
assumptions, would cover the underlying parameter in about 95% of repeated
experiments. It is not a 95% range for individual game outcomes. These intervals
do not account for every systematic effect of the opponent, hardware, time control,
opening choices, or excluded time losses. There is no fixed conversion here from
Stockfish's UCI_Elo setting to a person's FIDE rating.

Remus4-Rust shares Remus4's core chess algorithms but uses a different implementation
and a larger table. Equal-time searches can choose different moves. The rating gap
in this benchmark does not isolate speed, table capacity, or any one enhancement,
and does not establish a strength difference against human players.

The experimental engines, Remus5, Remus5-Rust, Remus6-Rust, and Remus7-Rust, each
played 20 games against the same limited-strength Stockfish at 15 minutes per side
with no increment, one game at a time. Each engine pondered on Stockfish's time and
used its default root lookups in the Lichess tablebase. Remus6-Rust and Remus7-Rust
ran with a 2048 MB table and 16 search threads across the whole processor. Each
engine first played one game with each color at 2200, 2400, 2600, 2800, 3000, and
3190, Stockfish's highest setting, and then eight games at the two levels 100 points
below and above the rating from those first twelve games. Twenty games give a much
wider interval than 200, so these rows place an engine on the scale only roughly.
Games lost on time are left out, as above.

In class, record wins, draws, losses, and unfinished games as observations. One
short round against a limited-strength Stockfish setting does not measure a rating.
Passing `verify_remus.py` checks its reference cases, not Elo, and games using the
registered instructor engines do not test the student pair's typed code.
