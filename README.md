# Remus

Remus4 plays chess with 630 lines of Python. It generates legal moves, scores
positions, and searches ahead with alpha-beta, which skips branches that cannot
change the best move. An opening book supplies stored opening lines. Playing
strength depends on the position, time control, and hardware.

The "Turning a Chess Engine into Science AI" workshop builds Remus one lesson
at a time in plain Python. The supplied Rust versions extend the same ideas.

<table>
  <tr>
    <td width="39%" valign="top">
      <a href="docs/lucas_chess_guide.md"><img src="utils/images/lucas_chess/14_game_running.jpg" alt="Remus2 plays Remus1 in Lucas Chess, with the board, both clocks, and the score of each move" width="100%"></a>
    </td>
    <td width="61%" valign="top">
      <a href="utils/mermaid/Remus2/modules.svg"><img src="utils/mermaid/Remus2/modules.svg" alt="The module call diagram of Remus2" width="100%"></a>
    </td>
  </tr>
  <tr>
    <td valign="top">Remus2 plays Remus1 in <a href="docs/lucas_chess_guide.md">Lucas Chess</a>.</td>
    <td valign="top">Inside Remus2: which function <a href="Day%202%20-%20Remus2%20searches%20ahead/Remus2/Remus2.md">calls which</a>.</td>
  </tr>
</table>

Its name reaches back more than four thousand years. Sargon of Akkad (about 2335 to
2279 BC) founded the Akkadian Empire in Mesopotamia, the first known great empire. In
1978 his name went to Sargon, a chess program by Dan and Kathe Spracklen. Written in
Z80 assembly for the Wavemate Jupiter II, Sargon won the first microcomputer chess
tournament, at the West Coast Computer Faire in 1978, with a perfect 5-0. The
Spracklens then published its complete source as a book, *Sargon: A Computer Chess
Program* (Hayden, 1978), so anyone could read a real engine and learn from it. Sargon
II, 2.5, and III followed, and Sargon III ran on the first Macintosh.

Sargon of Akkad's first son and heir was Rimush (about 2279 to 2270 BC), who held the
empire together through widespread revolts. Remus echoes Rimush: a "son of Sargon," an
heir to the small, readable chess programs whose insides lie open for learners,
rebuilt in modern Python.

## A strong chess engine in surprisingly little code

Build an opponent that can beat you. Remus starts with 189 lines of Python that
play legal chess. At 300 lines it searches ahead; at 630 lines it combines the
workshop's search and evaluation techniques with a recorded ELO estimate of 2195.
You can understand every part of an engine whose moves may be stronger than your own.

**TLOC means Total Lines of Code**: runtime code excluding blank lines, comments,
docstrings, and supplied verification programs.
The supplied data files do not have to be typed and don't represent code.

| Engine | ELO | Rank | Language | TLOC | What to explore |
| --- | ---: | --- | --- | ---: | --- |
| [Remus1](Day%201%20-%20Remus1%20plays%20legal%20chess/Remus1/Remus1.md) | 300 | Below Class D | Python | 189 | Legal moves and a random choice: the starting point |
| [Remus2](Day%202%20-%20Remus2%20searches%20ahead/Remus2/Remus2.md) | 1840 | Class A player | Python | 300 | Evaluation and alpha-beta search turn rules into decisions |
| [Remus3](Day%203%20-%20Remus3%20adds%20the%20biggest%20gains/Remus3/Remus3.md) | 2086 | Expert | Python | 484 | Remember positions, order moves, and use an opening book |
| [Remus4](Day%204%20-%20Remus4%20adds%20everything%20else/Remus4/Remus4.md) | 2195 | Expert | Python | 630 | Refine search, recognize draws, and improve endgame play |
| [Remus5](rust_engines/Remus5/remus5.md) | 2738 | Super Grandmaster | Rust | 1,849 | Run Remus4's core algorithms as compiled code with a larger table |
| [Remus6](rust_engines/Remus6/remus6.md) | 2809 | Super Grandmaster | Rust | 2,911 | Add positional terms, speed techniques, pondering, and tablebase advice |
| [Remus7](rust_engines/Remus7/remus7.md) | 3026 | Super Grandmaster | Rust | 3,330 | Use Lazy SMP (symmetric multiprocessing): search threads share a table |
| [Remus8](rust_engines/Remus8/remus8.md) | 3099 | Super Grandmaster | Rust | 3,941 | Use Young Brothers Wait Concept (YBWC): search the first move before splitting work |

ELO values estimate playing strength from benchmarks; Remus1 is a rough estimate.
Rank compares those numbers with human rating bands. These labels are not FIDE
titles or measured results against people.

Python makes the ideas approachable. Rust shows how much execution speed can add:
Remus5 retains Remus4's core algorithms and reaches a much higher benchmark rating
on one search thread. Remus7 and Remus8 explore the next step, parallel search;
their listed ratings use 16 threads and a 2 GiB table. The workshop defaults are
four search threads and a 512 MiB table for each engine on the 8 GB, four-core
laptops. The listed ratings do not measure this laptop configuration.
Speed and parallelism are central to Remus's progression, but algorithms,
evaluation, and hardware also matter. Parallelism is
not a prerequisite for the single-threaded Remus5 result.

**[Explore the full Remus version guide](docs/remus_versions.md)** for the enhancement
catalog, nodes-per-second comparisons, engine settings, and rating methods and
confidence intervals. Follow an engine link above to see how that version works.

## The workshop

The workshop is for students ages 10+ who have some coding experience and know
how to play chess. Students should understand functions, loops, and lists in a
language such as Java or Scratch, and recognize chess notation such as e4 and Nf3.
See the [workshop prerequisites](docs/workshop_syllabus.md#prerequisites).

Students work in ten student pairs over five days. Each day has four
90-minute periods. Students type Python from the period screenshots, check their
own code with the supplied verifier, and play their own engine in Lucas Chess.
On Day 5 they apply the same search ideas to science problems.

## Workspace layout

| Place | Contents |
| --- | --- |
| `Day 1 - ...` through `Day 4 - ...` | Day and period syllabuses, with one `RemusN` subfolder for that day's Python engine |
| Each `Day N - .../RemusN` | Python engine source, `RemusN.md`, `remusN.bat`, `verify_remusN.py`, and supplied data |
| `Day 5 - Solve a science problem with chess AI` | Day and period syllabuses and the two science starter programs |
| `rust_engines/Remus5` through `Remus8` | Rust source, ready-to-run Windows executables, and engine documentation |
| `docs` | Shared guides and briefs, containing only Markdown and PDF files |
| `utils` | VS Code extension installation, Lucas Chess engine registration, and other setup utilities |
| `utils/images` | Code screenshots, science figures, chess illustrations, and guide screenshots, grouped into named subfolders |
| `utils/mermaid` | Diagram sources, rendered SVGs, and manifests, grouped by engine or topic |

The [workshop syllabus](docs/workshop_syllabus.md) leads through each day.
The [version guide](docs/remus_versions.md) explains the engine progression.
Remus2 to Remus4 receive `piece_square_tables.json`; Remus3 and Remus4 also
receive `opening_book.txt`. Student pairs do not type those data files.

## Setup

To try Remus on your own Windows 11 PC, follow these steps. The repository includes
working Python engines and compiled Windows versions of the Rust engines, so you
can play them without typing the lessons or installing a Rust compiler.

1. Install Git, VS Code, and uv in PowerShell if they are not already installed:

   ```powershell
   winget install --id Git.Git -e
   winget install --id Microsoft.VisualStudioCode -e
   winget install --id astral-sh.uv -e
   ```

   Expect installation progress and completion messages. Resolve errors, then close
   and reopen PowerShell so it finds the installed commands. This installs tools;
   it does not test Remus.

2. Download the repository and enter its folder. Git calls this download a clone.
   No GitHub account is required:

   ```powershell
   git clone https://github.com/dbiersach/remus.git Remus
   cd Remus
   ```

   Expect download progress and a new Remus folder. Run the remaining commands
   from this folder. Keep the supplied engine source and data files in place.

3. Prepare Python and the editor extensions:

   ```powershell
   uv sync
   .\utils\install_vscode_extensions.ps1
   ```

   `uv sync` installs Python 3.14 if needed and creates the project's `.venv`
   environment with its dependencies. The extension installer prints progress
   messages for the VS Code tools used by this project. Neither command tests
   chess behavior. If PowerShell blocks the extension script, run
   `Set-ExecutionPolicy -Scope CurrentUser RemoteSigned` and retry it.

4. Install Lucas Chess from its [official download page](https://lucaschess.pythonanywhere.com/downloads)
   if it is not already installed. Use its default location under
   `%LOCALAPPDATA%\Programs\LucasChessR`. Open it, complete any first-run prompts,
   and wait for its main window so it creates its configuration files. Then close
   it before registering the engines:

   ```powershell
   uv run utils/install_lucas_engines.py
   ```

   Expect messages saying each Remus engine was added, updated, or already
   registered. The script connects Lucas Chess to the Python launchers in the Day
   folders and the executables under `rust_engines`. It disables Lucas Chess's
   external opening books for Remus so each engine uses its own opening choices.
   For Remus7 and Remus8, it sets Hash to 512 MiB and Threads to 4, replacing older
   saved values in their external-engine entries. Other saved options are kept;
   existing tournament entries have separate settings. Registration checks launch
   paths, not playing strength or chess correctness.

5. Open the workspace:

   ```powershell
   code Remus.code-workspace
   ```

   VS Code opens with the project interpreter and formatting settings. Open a Python
   file and confirm the Run Python File button is available. This checks editor setup.

6. Check Remus1 before playing:

   ```powershell
   uv run "Day 1 - Remus1 plays legal chess/Remus1/verify_remus1.py"
   ```

   Expect actual and expected results, followed by `Remus1 passes every test` and
   elapsed time. This checks reference move counts and legal UCI replies; it does
   not establish an Elo rating or prove every possible position correct. If it
   prints `PROBLEM:` or a traceback, resolve that error before playing.

7. Reopen Lucas Chess and confirm Remus1 through Remus8 under **External engines**.
   Select Remus1 as your opponent to try a game. Follow the
   [Lucas Chess guide](docs/lucas_chess_guide.md) for the game controls and engine
   settings. If an entry is missing, close Lucas Chess and rerun the registration
   command in step 4.

The supplied Python launchers run uv automatically. Keep the repository in the
same location after registration. If you move it, close Lucas Chess and rerun
`uv run utils/install_lucas_engines.py` from the new location to update the paths.

## Explore and modify the engines

Start with the Remus1 source and its guide, then compare each successive version.
The Day folders include lessons and code screenshots if you want a guided
walkthrough. You can also edit the supplied code directly and try your own ideas.
Before playing a modified Python engine, run its `verify_remusN.py` in the same
folder. Use `verify_remus2.py`, `verify_remus3.py`, or `verify_remus4.py` for the
corresponding version.

| What you run | Expected result | What it checks |
| --- | --- | --- |
| A helper module such as `make_move.py` | Exits without output, or shows a traceback | Syntax, imports, and any top-level initialization; it does not call the chess functions to test them |
| `uci.py` | Waits silently for UCI commands; no board window | Startup only; close that terminal when finished |
| The day's `verify_remusN.py` | Prints actual and expected results, then `RemusN passes every test` and elapsed time; Day 4 intermediate checks name the period | Reference move counts, UCI legal replies, and version-appropriate evaluation and search checks; not every position, Elo, or Lucas configuration |
| Lucas Chess with a completed Python engine | Board moves and clocks advance | The engine and launcher operate through the GUI for that game |
| A Day 5 science program | Prints measurements and displays graphs; close each graph to continue | Exercises the completed functions; compare results with the period instructions, since printed expectations are not automatic pass/fail assertions |

`PROBLEM:` or a traceback means a verifier has not passed. Fix the named problem
and rerun it before playing that version. Short games illustrate behavior; they do
not establish a dependable Elo rating or prove correctness for all positions.

To use your modified engines on another computer, copy the project with your
source and data files, then follow the setup steps there. Let `uv sync` create
that computer's Python environment rather than copying `.venv`.

## License

See [LICENSE](LICENSE).
