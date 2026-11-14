# Workspace reorganization handoff

This handoff is for the workshop designer and Claude Code. It records the implemented layout and the safeguards needed when continuing work. The repository root is `C:\DaveB\Dev\Remus`.

## 1. Python engines live with their workshop days

The complete student-style Python engines are retained temporarily for testing:

| Version | Folder |
| --- | --- |
| Remus1 | `Day 1 - Remus1 plays legal chess/Remus1` |
| Remus2 | `Day 2 - Remus2 searches ahead/Remus2` |
| Remus3 | `Day 3 - Remus3 adds the biggest gains/Remus3` |
| Remus4 | `Day 4 - Remus4 adds everything else/Remus4` |

Each contains its runtime modules, `RemusN.md`, `remusN.bat`, and `verify_remusN.py`. Remus2 to Remus4 also contain `piece_square_tables.json`; Remus3 and Remus4 contain `opening_book.txt`. These data files are supplied to student pairs, never typed.

The launchers use `uv run uci.py`. Lucas Chess points to these Day-local launchers so testing and workshop play exercise the same folder layout. A verifier checks reference cases directly through Python; it does not check the batch launcher or Lucas configuration.

The runtime modules are generated from the annotated instructor solutions. Do not edit generated student runtime code by hand. `admin/make_student_engines.py` compares generated code with the retained copies without creating a separate student-engine tree. Its `--populate-days` option refreshes the Day copies when authorized engine changes require that.

## 2. The instructor folder contains Python teaching material

`engines_instructor` became `instructor`. Its engine subfolders are only Remus1 to Remus4. They contain annotated solutions and version-specific launchers/verifiers. Each `RemusN_details.md` retains only these two sections:

- Line-by-line source reference.
- Source coverage and identity.

The other explanatory material from the old details pages was merged with the old engine README into each Day-local `RemusN.md`. Student-facing pages teach typing from screenshots and do not direct students to copy instructor source.

`science_instructor` became `instructor/science`. The Day 5 starter programs moved from `science` directly into `Day 5 - Solve a science problem with chess AI`.

## 3. Rust engines have a separate public home and new numbers

All Rust source, books, Cargo files, executables, overview guides, and source-reference guides live under `rust_engines`:

| Former identity | Current identity | Executable |
| --- | --- | --- |
| Remus4-Rust | Remus5 | `rust_engines/Remus5/remus5.exe` |
| Remus5-Rust | Remus6 | `rust_engines/Remus6/remus6.exe` |
| Remus6-Rust | Remus7 | `rust_engines/Remus7/remus7.exe` |
| Remus7-Rust | Remus8 | `rust_engines/Remus8/remus8.exe` |

Each folder contains `remusN.md` and `RemusN_details.md`. Students do not type Rust. Remus5 is the Rust implementation of workshop Remus4 with its larger fixed table. Remus6 adds the advanced single-threaded engine, tablebase boundary, and pondering. Remus7 uses Lazy SMP; Remus8 uses YBWC.

All four executables were rebuilt with their new identities. The reorganization does not introduce chess algorithm changes. The native verification suites passed for the rebuilt executables, using preserved baselines for the parallel versions.

The former Python Remus5 is preserved under `.archive/Remus5-Python`, including its original documentation. It remains a private historical reference for advanced Python/Rust boundary checks. It is not a current student engine or the current meaning of Remus5. Useful conceptual content is included in the current Rust documentation.

## 4. Day and shared documentation folders are simpler

Day and period syllabuses sit directly in their Day folders. There are no `period1` through `period4` directories. Each Python engine guide sits inside that day's engine folder.

`docs` is flat: shared Markdown guides, the two PDF briefs, and `install_lucas_engines.py` live directly there. The redundant `docs/README.md` is removed.

Images live only under the root `images` asset tree, grouped into Day1 through Day5, chess_tutor, lucas_chess, and color_swatches. Period prefixes distinguish files that formerly occupied separate period directories.

Mermaid sources, rendered diagrams, and manifests live under root `mermaid`, grouped by Remus version or chess_tutor. Markdown links and the asset generators use those locations.

The code screenshots were checked against the generated period source. The Lucas engine-list and Rust-path screenshots show current identities. Existing game illustrations remain examples; no games were started to make screenshots.

## 5. Student preparation is a deliberate final step on a clone

`setup_workshop.ps1 -StudentCopy` is for a disposable student clone, never the designer workspace. It refuses to run where `admin/workshop_paths.py` identifies the private designer installation.

The script validates required supplied files, runs `install_vscode_extensions.ps1`, and runs `uv sync`. Only after successful setup does it remove:

- Runtime `.py` modules from the four Day-local engine folders, except `verify_remusN.py`.
- Python bytecode caches in those engine folders.
- Git history, instructor solutions, archives, designer tools, temporary files, and private tool metadata.

It keeps the engine guides, batch launchers, version verifiers, JSON data, opening books, images, Mermaid assets, Rust engines, and Day 5 starter programs. It writes a preparation marker and deletes itself after success. A failure before cleanup retains the files and setup script for retry; `-WhatIf` makes no changes. The marker prevents accidental repeat preparation from deleting work students have typed.

This designer checkout intentionally retains the complete Day-local Python runtime files until testing is finished. Do not remove them here as part of routine generation or documentation work.

## 6. Documentation distinguishes checks from play

The syllabuses, READMEs, guides, and PDF briefs distinguish three operations:

1. Running a module checks whether that module and its imports load; it does not call and validate all its functions.
2. Running `verify_remusN.py` checks the supplied reference cases for that version, including legal UCI replies and terminal positions.
3. Playing through Lucas Chess exercises the student's engine and launcher for positions reached in that game. A short game or round does not establish Elo or isolate a speed improvement.

`docs/remus_versions.md` contains the language, lines-of-code, estimated Elo, approximate human level, and engine defaults table. Rating-method and uncertainty discussion appears in the closing section of `docs/remus_versions.md`; `docs/fide_titles_and_elo_ratings.md` describes human titles and rating ranges. Human levels are approximate descriptions, not earned FIDE titles.

Runtime Python counts are Remus1: 189; Remus2: 300; Remus3: 484; Remus4: 630. The heading is “Lines of code”; there is no physical-lines column. Verifiers are supplied tools and are excluded from runtime code counts and typing tables.

## 7. Designer tools use shared path helpers

`admin/workshop_paths.py` centralizes current Day, instructor, Rust, guide, and diagram paths. Generators, verifiers, build tools, rating tools, and diagnostic tools use the current layout. `admin/README.md` inventories all 61 active top-level Python tools.

Dedicated YBWC tools are named for Remus8; the single-threaded advanced diagnostics tool is named for Remus6. Some historical fixture tools deliberately retain their legacy numeric arguments and translate them through explicit compatibility helpers. Read the compatibility section in `admin/README.md` before invoking those tools; a legacy argument does not rename the current engines.

Historical PGNs, frozen benchmarks, and their recorded engine names remain unchanged. Rating readers translate recognized historical identities in memory. New approved rating games use current engine names. Do not indiscriminately replace old names inside preserved evidence.

Useful current commands, from the repository root:

```powershell
uv run admin/make_student_engines.py
uv run admin/make_code_images.py --check
uv run admin/verify_engines.py
uv run admin/verify_student_uci.py
uv run admin/verify_engine_docs.py
uv run admin/verify_mermaid_colors.py
uv run admin/build_rust.py --engine 5
uv run admin/verify_rust.py --engine 5
```

Build Rust engines sequentially. Engines 7 and 8 need the appropriate preserved baseline executable for comparative `verify_rust.py --baseline` checks. The commands above do not start tournament games.

## 8. Lucas Chess configuration follows the new paths

The external-engine list has eight entries, Remus1 through Remus8. Python entries point to the Day folders; Rust entries point to `rust_engines`. Saved options and custom aliases are preserved. Remus GUI opening books remain disabled so engines use their own supplied books.

`docs/install_lucas_engines.py` registers current engines and handles Lucas's cached option containers without adding duplicates. It checks paths and saves configuration; it does not prove that an engine plays correctly. Run it with Lucas closed.

`admin/migrate_lucas_layout.py` is a private preview/apply tool for existing configurations. The applied migration backed up external settings and three tournament databases, changed engine paths and matching labels, and preserved all non-engine tables and option values. Historical tournament names and game records retain their recorded identities. The archived Python5 entry is removed from the active external list but preserved in the backup; historical tournament entries can still identify the archived engine.

Lucas is left open at its main window. No workers or games were launched.

## 9. Cleanup preserves evidence

The unused `.venv-ft` is removed; the main `.venv` remains. `.workshop-designer` is removed because the setup guard uses the private path helper. There is no active `engines_student` folder; its prior content is preserved privately.

Temporary build trees and generated previews were cleaned only after every file was archived and hash-verified. The backup is `admin/tmp_cleanup_20261114/complete_build_backup.zip`, with `complete_manifest.json` recording the files. It preserves 39,027 files from 43 removed roots. About 9.07 GiB of generated files were removed and a 3.64 GiB backup retained, freeing about 5.4 GiB net. The remaining `tmp` content is about 0.83 GiB. Reports, configuration backups, and frozen experiment evidence remain available.

General migration backups and logs are under `admin/layout_migration_20261114`; the original tree snapshot is under its `initial` folder. `moves.json` records path moves. Lucas configuration backups are under `admin/lucas_layout_backups`. Do not rerun historical migration scripts indiscriminately: several are one-time transformations.

## 10. Validation and remaining limits

Completed checks include:

- Full Python native-engine verification and all four Day-local version verifiers.
- Student UCI reference checks, including special moves and no-legal-move positions.
- Generated student-code identity and period screenshot/reference checks.
- Rebuilt Rust5 through Rust8 native checks; parallel engines compared with preserved baselines.
- Engine-guide counts/defaults/source diagram checks and Mermaid color checks.
- Local links across all active Markdown pages, raster format/open checks, and visual inspection of both one-page PDF briefs.
- Repository Ruff checks and focused import/undefined-name checks across active designer scripts.
- Disposable setup tests covering success, failure, WhatIf, and designer-workspace protection.
- Lucas installer repeat-registration and cached-option preservation fixtures; historical rating-name parsing without changing PGNs.
- Preservation comparisons for all non-engine tables and saved engine options in the three migrated Lucas tournament databases.

These checks establish path consistency and tested behavior, not an Elo conclusion. Original-versus-shortened matches remain paused at 289 of 800 games. Their evidence is under `admin/workshop_reduction_20261113/matches_60s`, with `pause_state.json` recording the pause. Do not resume those or any other games without David's instruction.

The reorganization is recorded as one coherent layout change. Git may report many deletions and additions before it detects the moves; do not discard them as accidental churn. Private designer tools, archives, backups, and paused-match evidence remain outside Git under the existing ignore rules.
