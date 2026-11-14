# Lucas Chess: Play and Watch Remus

Remus has no board window of its own. It reads moves as text and answers with text.
**Lucas Chess** is the chess app on every class laptop, and it gives Remus a board,
pieces, and clocks. Lucas Chess starts Remus through the file `remus.bat` and talks
to Remus in UCI, the Universal Chess Interface: Lucas sends `position startpos moves
e2e4` and `go`, and Remus answers `bestmove c7c5`. [`uci.md`](../chess_tutor/uci.md) explains that
conversation line by line.

This guide shows four things:

1. Start Lucas Chess.
2. Find the engines in Lucas Chess.
3. Play a game against Remus1, as each student does at the end of Day 1.
4. Run tournaments between the versions: Remus2 against Remus1, Remus3 against Remus2,
   and Remus4 against Remus3.

The class entries point to `engines_instructor/`. Games here exercise those
instructor engines, not the code student pairs type in `engines/`. To check a
student copy, open that version's `verify_remus.py` in VS Code and press Run Python
File at the scheduled checkpoint. Expect actual and expected results, followed by
`RemusN passes every test` and elapsed time (an intermediate Remus4 check names its
period). `PROBLEM:` or a traceback means the check has not passed. The verifier
covers reference cases, not every position or function branch.

## Part 1: Start Lucas Chess

Click the Windows **Start** button, then **All**, and find **Lucas Chess (R)** under
the letter L. You can also double-click the Lucas Chess icon on the desktop.

Always start Lucas Chess one of these two ways. Both start it from its own folder,
and Lucas needs that folder to run engine-against-engine games.

![The Lucas Chess main window](jpg/01_main_window.jpg)

The buttons along the top open menus. This guide uses **Play** and **Engines**.

## Part 2: Find the engines

Every class laptop already lists the Remus engines, because step 6 of the Day 1
setup runs [`install_lucas_engines.py`](install_lucas_engines.py). Nobody adds an
engine by hand. During setup, the installer prints an entry for each engine and
ends with `Saved ... engines in ...` or `Nothing changed: every engine is already
registered`. A missing installation or file message means it has not completed.
Start Lucas Chess once to create its configuration, then close it before running
the installer. Registration saves paths and settings; it does not launch or test
the engines.

**Step 1.** Click **Engines**, then **External engines**.

![The Engines menu](jpg/02_engines_menu.jpg)

**Step 2.** The window lists Remus1, Remus2, Remus3, Remus4, and Remus4-Rust. Click
**Close**. Seeing a name confirms a saved entry, not that its program can play.
If a name is missing, follow the setup registration step before continuing.

![The Remus engines in the list](jpg/03_external_engines.jpg)

## Part 3: Play against Remus yourself

**Step 1.** Click **Play**, then **Play against an engine**.

![The Play menu](jpg/04_play_menu.jpg)

**Step 2.** Under **Opponent**, click the wide button with the engine's name, then
choose **External engines** and **Remus1**. Under **Side you play with**, pick White,
Black, or Random.

![Choose Remus as your opponent](jpg/05_choose_remus.jpg)

**Step 3.** Open the **Time** tab and tick **Activate the time control**. Set **Total
minutes** to 10 and **Seconds added per move** to 0, so each side has 10 minutes for
the whole game.

![Turn on the clock](jpg/06_time_control.jpg)

**Step 4.** Click **Accept**, then **Continue**. If you chose White, drag a piece
to make your first move; if you chose Black, wait for Remus to move first. Expect
Remus's move on the board and in the move list, with the clocks changing sides.
This exercises the GUI-to-engine exchange for this game, not the student pair's code.

![A game against Remus1](jpg/07_playing_remus.jpg)

The screenshot shows White playing e4, Nc3, and Nf3, with Remus1 answering Nf6, g6,
and c5. These are examples, not expected test moves: Remus1 chooses randomly and
usually answers immediately. Later versions can take time to search, while Remus3
and Remus4 can answer immediately from their opening books. Move times vary with
the position and clock; an immediate reply alone does not prove a book was used.

Lucas has a tutor, and the bar under the moves shows that it is on. When the tutor
finds a better move than yours, a small menu opens beside the board. Click **Select my
move** to keep your move, or **Try again** to take it back.

To end a game early, click **Cancel** and answer **Yes**, then click **Close**.

Days 1 and 2 schedule a student round against the instructor's finished version;
Days 3 and 4 include it only if time remains. Each round has two games, one per
student, with one student playing White and the other Black. Record wins, draws, losses, and unfinished
games separately. These observations do not establish a reliable strength rating.

## Part 4: Run a tournament between two versions

Lucas calls a match between engines a "tournament", even when it has only two players.
The class runs three of them, one each time a new version is finished:

| Tournament | New engine | Previous engine |
| --- | --- | --- |
| `Remus2 vs Remus1` | Remus2 | Remus1 |
| `Remus3 vs Remus2` | Remus3 | Remus2 |
| `Remus4 vs Remus3` | Remus4 | Remus3 |

Both engines must be on the list from Part 2. These steps build the first tournament,
and the other two go in the same way.

**Step 1.** Click **Engines**, then **Tournaments between engines** (the last line of
the menu in the picture from Part 2). Click **New**, type the name `Remus2 vs Remus1`,
and click **Accept**.

![Name the new tournament](jpg/08_new_tournament.jpg)

**Step 2.** The **Configuration** tab opens. Lucas can stop a game early and give the
win to whichever side is far ahead. To watch every game to the end, set **Minimum
centipawns to assign winner** to `0` and **Minimum movements to assign draw** to `0`.
A centipawn is one hundredth of a pawn, so 350 centipawns means "ahead by three and a
half pawns".

![Set both adjudication boxes to 0](jpg/09_play_to_the_end.jpg)

**Step 3.** Open the **Engines** tab and click **Import**. Choose **External
engines**, then **Remus2**. Click **Import** again, and choose **External engines**,
then **Remus1**.

![Import the engines](jpg/10_import_remus.jpg)

**Step 4.** The tab shows two engines. Click a name to see its settings on the right.
For each Remus entry, use **Modify** and set **Opening book** to **`<None>`**,
not **`<By default>`**, so Lucas does not choose its moves from a GUI book. Saved
settings confirm configuration only; the engine is exercised when a game runs.

![Both engines in the tournament](jpg/11_two_engines.jpg)

**Step 5.** Open the **Games queued** tab and click **New**. Set **Rounds** to 1,
**Total minutes** to 10, and **Seconds added per move** to 0, and click **Accept**. Each
engine gets 10 minutes on its clock for the whole game.

![Choose the rounds and the clock](jpg/12_new_games.jpg)

**Step 6.** One round makes two games, so each engine plays once with White and once
with Black. The **Time** column shows 600 seconds.

![Two games wait to be played](jpg/13_games_queued.jpg)

**Step 7.** Click **Launch workers** at the top of the window and choose **2**. A
worker is another Lucas window that plays one game at a time, so two workers play both
games of the round at once.

![Remus2 plays Remus1](jpg/14_game_running.jpg)

Expect each worker window to show a board, clocks, and a growing move list. The
number after a move is the engine's estimate in pawns; +1.80 estimates an advantage
for White, not a proven result. Remus2 wins material in this screenshot, but the
same moves and winner are not guaranteed in another game. Remus1 reports +0.00
without evaluating the position, so its displayed score is not evidence of equality.

A game lasts up to 20 minutes, and the two games run side by side, so the round lasts
up to 20 minutes. When both games are over, the **Games finished** and **Results** tabs
of the tournament show who won. To stop early, click **Cancel** in each worker window;
the unfinished games stay in **Games queued**.

**Step 8.** Build the other two tournaments the same way when their versions are
finished, and launch two workers for each. Remus3 and Remus4 use their supplied
opening books when a legal continuation is available. The box at the bottom right
can show the engine's predicted line; it is neither a guaranteed continuation nor
proof that a particular move came from the book.

![Remus3 plays Remus2](jpg/15_remus3_vs_remus2.jpg)

![Remus4 plays Remus3](jpg/16_remus4_vs_remus3.jpg)

The list of tournaments keeps each one, so you can open it later to read its results.
Compare recorded wins, draws, and losses, keeping unfinished games separate. One
round illustrates play; it does not establish an Elo rating, prove one version is
stronger, or isolate an enhancement's effect. Reported depths in different game
positions are not a controlled speed comparison.

![The three tournaments in the list](jpg/17_three_tournaments.jpg)

### A tournament against Stockfish 1500

This tournament comes last in Period 4 of Days 2 and 3, after the round against the
previous version and the round against the student pair, and a class short of time
skips it.

Stockfish is one of the strongest chess engines in the world. Lucas Chess comes with
a Stockfish with a limited-strength setting labeled 1500 Elo. This is an opponent
setting, not a measured rating for Remus or a guaranteed result for any game. To use it as an
opponent, click **Import** in Step 3 and choose **Engines with limited elo**, then
**1500**, then **Stockfish 19 (1500)**. Click its name in the **Engines** tab, and its
ELO line reads 1500. The rest of the tournament goes the same way: one round, with 10
minutes for each side, and two workers.

### A tournament against Remus4-Rust

On Day 4 the second tournament of Period 4 plays Remus4 against Remus4-Rust, right after
Remus4 against Remus3. Remus4-Rust is the instructor's Remus4 written in Rust, with the
same core evaluation and search, a different implementation, and a larger table.
Observe reported depths and game results; this short round does not isolate the
contribution of speed from that of the table or establish a strength difference. Click **Import** in Step 3, choose **External engines**, then
**Remus4-Rust**. The rest of the tournament goes the same way: one round, with 10
minutes for each side, and two workers.

![Remus4 plays Remus4-Rust](jpg/18_remus4_vs_rust.jpg)

The box at the bottom right shows the line Remus4-Rust expects, from move 8 to move 14
in this picture.

## Lucas Chess at home

Lucas Chess is free. To use it on your own computer at home, download the installer
from the official site, [lucaschess.pythonanywhere.com/downloads](https://lucaschess.pythonanywhere.com/downloads),
and run it. It installs like any other Windows program, and it comes with Stockfish,
so every tournament in Part 4 works the same way at home. Remus and its `remus.bat`
launcher run on Windows only.

To play your own Remus at home, copy the Remus folder from your USB drive to a Windows
computer, install `uv`, and run `uv sync` once in the Remus folder, the same setup as
on Day 1 (see the setup steps in the top `README.md`). In Lucas Chess, click **Engines**,
**External engines**, and **New**. The window that opens shows only `.exe` files, so
paste the full path of your `remus.bat` into the **File name** box and click **Open**.
Choose the file in the student pair's own folder, for example
`Remus/engines/Remus4/remus.bat`, rather than the instructor folder. Give the entry
a distinct name such as **My Remus4**, then click **Accept**. Expect that entry in
**External engines**; registration alone does not test its chess behavior. First
run `verify_remus.py` in the same student engine folder and check its success line.
Then select **My Remus4** in Part 3: the moves should appear on the board, exercising
your own copy through Lucas Chess for that game.

## When something goes wrong

| What you see | What to check |
| --- | --- |
| Lucas shows an error when you add a workshop Python engine | Open `uci.py` beside the selected `remus.bat` and press Run Python File. A load error shows a traceback naming the file and line. Successful startup waits silently for commands; it does not test chess functions. Close that run's terminal before retrying in Lucas. |
| A workshop Python engine never moves | Open `verify_remus.py` in the engine folder selected by Lucas and press Run Python File. Require its success line; fix any `PROBLEM:` or traceback. Its UCI check looks for the name and handshake replies, then checks the returned `bestmove` against the fixed set of 20 legal Black replies to `1. e4`, independently of the student move generator. It does not test every position or book choice. If it passes but Lucas still fails, check the saved engine path and project setup; the verifier launches `uci.py` directly and does not test `remus.bat` or Lucas's configuration. |
| Remus4-Rust cannot start | Confirm the entry points to `engines_instructor/Remus4-Rust/remus4_rust.exe`, not a Python launcher. The workshop Python verifier does not test this executable. Report a persistent startup error to the teacher; opening a console window alone is not a chess test. |
| **Launch workers** does nothing | Quit Lucas Chess and start it again from the Start menu or the desktop icon, then reopen the queued tournament. Launching workers should open board windows and advance the move lists. If it does not, the games have not started; this is not a failed chess result. |

## Experimental engine settings

Remus4-Rust, Remus5, and Remus5-Rust each use a fixed 512 MiB search table without a Hash option.
Remus6-Rust and Remus7-Rust offer Hash sizes 512, 1024, or 2048 MiB,
with default 512 MiB. Python Remus3 and Remus4 use two-million-entry dictionaries;
Remus1 and Remus2 have no table.

Remus6-Rust and Remus7-Rust default to four search workers. Each engine's workers
share one table. Two engines with Hash 512 allocate 1 GiB for search tables, plus
other engine memory and Lucas Chess. Each simultaneous tournament game adds two
engine allocations. A saved explicit Threads setting takes precedence over the
engine's advertised default.

Enable Ponder in the external and tournament entries for Remus5, Remus5-Rust,
Remus6-Rust, and Remus7-Rust. Their advertised default is false, so Lucas records
the enabled selection and starts its pondering controller. The engine only thinks
on the opponent's time when the GUI sends `go ponder`. Selecting the option or
seeing a predicted reply does not confirm that pondering occurred. The experimental
engine READMEs describe optional diagnostics for observing those commands and timing;
those logs are measurements, not automatic pass/fail checks.

## Remus opening books

Every Remus engine uses only its supplied `opening_book.txt` for opening-book
moves. Python reads the text file; Rust embeds it when the executable is built.
A Rust book edit therefore requires rebuilding the executable. Remus1 and Remus2
have no opening book. Other versions use native search when the file supplies no
legal continuation.

For tournament engines, select **Engines**, the Remus entry, and **Modify**.
Set **Opening book** to **`<None>`**, not **`<By default>`**. This disables Lucas
Chess's external book for that engine, even if the tournament has a default book.
It does not disable the book inside Remus. The book-selection **Mode**, such as
**Proportional random**, has no effect without a Lucas-supplied book.

This setting applies to Remus5, Remus5-Rust, Remus6-Rust, and Remus7-Rust as well as the
workshop engines. Imported tournament entries carry their saved settings, so
check **Opening book** when importing an entry. In **Play against an engine**,
leave the opponent's **Activate book** option unchecked and do not force an
opening line when Remus should choose its own opening moves.
