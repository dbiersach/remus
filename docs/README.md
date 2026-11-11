# docs

The pages people read during the workshop, beside the plan of the workshop in
`workshop_syllabus.md` and the day folders at the top of the repository.
Students, teachers, and parents open these pages in VS Code or on GitHub. None of them
holds code to type or run.

| Folder | What it holds | Who reads it |
| --- | --- | --- |
| [`engines/`](engines/) | One folder per engine, `Remus1` to `Remus4`, each with a `README.md` (its files, their code lines, and what the version adds) and Mermaid call diagrams, with editable sources and rendered SVGs in `mermaid/`. [`remus_versions.md`](engines/remus_versions.md) lists every enhancement and the version that adds it. [`Remus4-Rust`](engines/Remus4-Rust/README.md) describes Remus4 written in Rust. [Remus5](engines/Remus5/README.md), [Remus6](engines/Remus6/README.md), and [Remus6-Rust](engines/Remus6-Rust/README.md) describe experimental instructor extensions outside the workshop. Each engine README describes its cache capacity. | Student pairs, before and while they type a version |
| [`lucas_chess/`](lucas_chess/) | The [Lucas Chess guide](lucas_chess/lucas_chess_guide.md) and its 18 screenshots in `jpg/`: how to find the Remus engines, play against one, and run tournaments between the versions. | Student pairs and teachers |
| [`chess_tutor/`](chess_tutor/) | [`learn_chess.md`](chess_tutor/learn_chess.md), the rarer rules (castling, en passant, promotion, the draws) and how moves are written, with its board pictures in `images/`; [`uci.md`](chess_tutor/uci.md), the conversation between Lucas Chess and Remus; [videos](chess_tutor/minimax_negamax_alpha_beta_video_resources.md) on minimax, negamax, and alpha-beta; and [`fide_titles_and_elo_ratings.md`](chess_tutor/fide_titles_and_elo_ratings.md), the FIDE titles and their typical Elo ratings. | Student pairs, right before the code that needs each idea |
| [`briefs/`](briefs/) | Two one-page PDFs: the [management brief](briefs/Turning_a_Chess_Engine_into_Science_AI_Management_Brief.pdf) and the [student and parent brief](briefs/Turning_a_Chess_Engine_into_Science_AI_Student_and_Parent_Brief.pdf). | Managers, students, and parents, before the workshop |

The engine code itself lives in the top `engines_instructor/` folder, the student
pairs' own engines in `engines/`, the Day 5 programs in `science/`, and their finished
versions in `science_instructor/`.
