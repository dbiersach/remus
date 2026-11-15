# Instructor guide: prepare each student laptop

Complete these steps on every workshop laptop before Day 1, using the Windows
account the student pair will use. The workshop is for students ages 10+ who
have some coding experience and know how to play chess. Students arrive to a
working, offline environment; they do not install tools or create online accounts
in class.

The [main README](../README.md#setup) describes home use with completed engine
source. This guide adds the instructor's checks and the student cleanup step.
Cleanup removes the completed Python solutions so student pairs can write their
own modules from the lesson screenshots. Do not use it on your development
checkout or on a folder containing students' work.

## 1. Prepare the Windows account and laptop

Use Windows 11 and the intended classroom account, with permission to install
software. The workshop configuration targets 8 GB of RAM and four CPU cores.
Connect power and internet for preparation. Install pending operating-system
updates and finish required restarts before proceeding.

Choose a permanent, writable workspace location, such as the account's Documents
folder. Lucas Chess stores absolute engine paths, so moving the workspace later
requires registering the engines again. Keep the instructor's development copy
and any backup of completed solutions off the student laptop.

Record the laptop identifier, Windows account, workspace path, and preparation
date in your instructor records. Check that the display comfortably shows code
screenshots beside the editor; students must be able to read them without copying
source text.

## 2. Install Git, VS Code, and uv

Open PowerShell under the classroom account and run:

```powershell
winget install --id Git.Git -e
winget install --id Microsoft.VisualStudioCode -e
winget install --id astral-sh.uv -e
```

Expect installation progress and completion messages. An already-installed
message is acceptable. Resolve installation errors, then close and reopen
PowerShell so its command search path includes the tools. Check:

```powershell
git --version
code --version
uv --version
```

Expect version numbers, not a command-not-found error. These checks establish
that the commands can start, not that Remus works. Python is supplied through
uv; a separate Python installer and a Rust compiler are unnecessary for the
workshop's supplied programs.

If PowerShell refuses to run the supplied scripts, use:

```powershell
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned
```

Then retry the blocked script. If institutional policy controls this setting,
ask the laptop administrator to permit the workshop scripts; do not bypass the
institution's policy.

## 3. Install and initialize Lucas Chess

Download Lucas Chess from its [official download page](https://lucaschess.pythonanywhere.com/downloads).
Install it under the classroom account in its default location:
`%LOCALAPPDATA%\Programs\LucasChessR`.

Open Lucas Chess, complete any first-run prompts, and wait for the main window.
This is what "initialize" means here: Lucas creates its local configuration
files. Close Lucas Chess completely before running Remus setup. Merely downloading
or installing Lucas is not enough to create that configuration.

The setup script expects `bin\LucasR.exe` and the `UserData\__Config__` folder
under that installation. It stops with an explanation if Lucas is absent,
uninitialized, or still running. Preparing a different Windows account does
not prepare the classroom account's Lucas configuration or VS Code extensions.

## 4. Download a separate classroom copy

In PowerShell, change to the parent folder you chose, then run:

```powershell
git clone https://github.com/dbiersach/remus.git Remus
cd Remus
git rev-parse HEAD
```

Expect Git download progress and a new `Remus` folder. The last command prints
the revision identifier; record it with the laptop details before cleanup
removes Git metadata. No GitHub account is required for this public download.
Use the same approved revision on all classroom laptops.

Run the remaining commands from this `Remus` folder. The download includes
completed Python testing copies and compiled Rust executables. The student setup
step removes the completed Python modules after installation and registration.
Do not copy your author workspace, virtual environment, or personal Lucas
configuration onto a classroom laptop.

## 5. Check the supplied programs before cleanup

Prepare this laptop's Python environment:

```powershell
uv sync
```

Expect dependency installation and a local `.venv` folder. uv obtains Python
3.14 if needed. This downloads the science and verification dependencies as well
as preparing the interpreter; it does not test chess behavior.

While the completed Python copies are present, check all four versions:

```powershell
uv run "Day 1 - Remus1 plays legal chess/Remus1/verify_remus1.py"
uv run "Day 2 - Remus2 searches ahead/Remus2/verify_remus2.py"
uv run "Day 3 - Remus3 adds the biggest gains/Remus3/verify_remus3.py"
uv run "Day 4 - Remus4 adds everything else/Remus4/verify_remus4.py"
```

Each command prints actual and expected results, then `RemusN passes every test`
and elapsed time. Require the success line for each version. A `PROBLEM:` message
or traceback needs investigation before cleanup. These checks exercise reference
move counts, legal UCI replies, and version-appropriate evaluation/search cases.
They do not test the Lucas launcher or establish an Elo rating. Keep the results
with your instructor records outside this disposable preparation folder.

Check each supplied Rust executable without starting games:

```powershell
foreach ($number in 5..8) {
    Write-Host "Checking Remus$number: expect its name, uciok, readyok, and perft 400"
    $enginePath = ".\rust_engines\Remus$number\remus$number.exe"
    @('uci', 'isready', 'position startpos', 'go perft 2', 'quit') | & $enginePath
    if ($LASTEXITCODE -ne 0) { throw "Remus$number did not exit successfully." }
}
```

Inspect each result for the expected identity and replies. `perft 400` counts the
starting position's legal two-ply continuations. This is a small rules check,
not a full search test; the loop's exit-code check does not itself compare the
printed count. The engines run sequentially and exit on `quit`.

## 6. Run student setup

Keep Lucas Chess closed. Preview the planned cleanup first:

```powershell
.\utils\setup_student.ps1 -StudentCopy -WhatIf
```

Expect a list of paths that would be removed and a description of the setup
steps. This preview does not install tools, register engines, or delete files;
it also does not verify the actual Lucas installation. Confirm that the displayed
paths belong to this classroom copy.

Then run:

```powershell
.\utils\setup_student.ps1 -StudentCopy
```

The script performs these operations in order:

1. Checks required workspace files, engine launch paths, and cleanup boundaries.
2. Requires Lucas Chess to be installed, initialized, and closed.
3. Runs `install_vscode_extensions.ps1` for the classroom account.
4. Runs `uv sync` again to ensure the environment matches the project.
5. Runs `install_lucas_engines.py` and requires registration to succeed.
6. Removes temporary runtime `.py` files from the four Day engine folders,
   keeping each `verify_remusN.py`.
7. Recursively removes `instructor`, Git metadata, archives, and private designer
   material that is present in the copy.
8. Writes `.workshop-student.json`, prints `Workshop setup complete`, and deletes
   `setup_student.ps1` itself on success.

Registration messages name the engines and end with `Saved ... engines in ...`
or `Nothing changed: every engine is already registered`. Registration saves
paths and options; it does not play or verify an engine. It sets Remus7 and
Remus8 to **Hash = 512 MiB** and **Threads = 4**, overriding older saved values
for those external-engine entries. Existing tournament entries are separate.

If setup stops, read the error, resolve it, and rerun only while the script
remains and preparation is incomplete. Do not call a failed run complete because
some extensions installed. The cleanup phase is not a rollback transaction:
if deletion itself fails, inspect what remains before retrying. Never restore
and rerun cleanup over student work.

## 7. Inspect the classroom workspace and editor

Open the workspace:

```powershell
code Remus.code-workspace
```

VS Code should open the full workspace with its `.venv` interpreter. Confirm
that Python, Pylance, Ruff, Markdown preview, and PDF viewing are available.
The extension list also includes optional AI tooling; no AI account or sign-in
is required for the workshop. Student pairs type the displayed lessons themselves.

Check the following files and folders in Explorer:

| Location | Expected contents after setup |
| --- | --- |
| Each Day 1 to Day 4 `RemusN` folder | `RemusN.md`, `remusN.bat`, and `verify_remusN.py`; no completed engine modules |
| Remus2 through Remus4 | Supplied `piece_square_tables.json` |
| Remus3 and Remus4 | Supplied `opening_book.txt` |
| Day 5 | `hidden_peak.py` and `phase_hunt.py` starter programs, plus syllabuses |
| `rust_engines/Remus5` through `Remus8` | Executables, Rust source, and documentation |
| `utils/images` and `utils/mermaid` | Lesson screenshots and diagram assets |
| Workspace root | `.venv`, `Remus.code-workspace`, and `.workshop-student.json` |
| Removed items | `instructor`, `.git`, and `utils/setup_student.ps1` are absent |

Open the Day 1 period syllabus in Markdown preview and check its code screenshot.
Open the guides for the other days and a PDF brief. Follow their local links to
confirm that the lessons and assets are accessible. Missing screenshots or broken
local links are preparation problems, not student coding exercises.

After cleanup, running `verify_remus1.py` cannot pass until students create the
engine modules. Missing-module feedback at this point is expected. Do not restore
solutions just to make that verifier pass on the prepared classroom copy.

## 8. Check Lucas registration and classroom settings

Reopen Lucas Chess and choose **Engines > External engines**. Confirm all eight
Remus names and inspect their program paths:

| Engines | Program path relative to the permanent Remus folder |
| --- | --- |
| Remus1 through Remus4 | Their corresponding `Day N - ...\RemusN\remusN.bat` |
| Remus5 through Remus8 | `rust_engines\RemusN\remusN.exe` |

Remus1 through Remus4 become playable after the student pair types and verifies
its code. Their saved entries are intentionally present before that work exists.
The Rust executables are supplied ready to run.

Require **Hash = 512** and **Threads = 4** for Remus7 and Remus8. Remus5 and
Remus6 have fixed 512 MiB tables. Check **Ponder = False** for Remus6 through
Remus8 unless a lesson explicitly requires pondering, and leave diagnostics off
for ordinary classroom play. For fully offline classroom use, set
**LichessTablebase = False** for Remus6 through Remus8. Native search works without
this optional internet service. The installer preserves these other saved
options, so inspect them rather than assuming registration resets everything.

Disable Lucas external opening books for every Remus entry. In tournament
entries, choose `<None>`, not `<By default>`. Check existing tournament
entries separately because registration does not update their stored options.
Each concurrently running engine process allocates its own table and workers;
four threads is a per-engine setting, not a laptop-wide limit.

Confirm that the bundled Stockfish opponent is available for the scheduled
comparisons. Follow the [Lucas Chess guide](lucas_chess_guide.md) for game setup:
a student pair plays together in one 10+0 game against Remus and chooses White
or Black. Engine comparisons use the scheduled opposite-color games. These
settings prepare future classroom games; installation checks do not require a
tournament or an Elo measurement.

## 9. Check offline readiness and hand over the laptop

Disconnect the laptop from the network temporarily and run:

```powershell
uv run --offline python -c "import sys, chess, matplotlib; print(sys.executable); print('Verification and plotting dependencies import successfully')"
```

Expect the interpreter path inside this workspace's `.venv` and the printed
success message. This checks offline dependency availability, not chess or
science answers. Reopen the workspace and Lucas Chess while offline, inspect
lesson images, and repeat the Rust protocol check from step 5 if needed.
Resolve any missing-download prompt while internet is available, then repeat
the affected offline check.

Leave the workspace ready at Day 1, with the intended classroom account signed
in and no completed Python solutions available locally. Record completion,
settings, and any unresolved issue for each laptop. All installation and
registration should be finished before students arrive.

If a registered path needs repair later, close Lucas Chess and run
`uv run utils/install_lucas_engines.py` from the workspace. Expect registration
messages, then reopen Lucas to inspect the entries. This repairs paths without
removing students' code. Home users who want completed examples should follow
the main README instead of the student cleanup procedure.
