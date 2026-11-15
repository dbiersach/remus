# CLAUDE.md - Remus

Claude Code does not read `AGENTS.md` natively, so this file imports it. The line below
pulls in the full project rules and style guide.

@AGENTS.md

Keep the `@AGENTS.md` import relative to this file at the repository root. The
current layout and student setup workflow are maintained in
[AGENTS.md](AGENTS.md#project-setup-and-layout) and [README.md](README.md#setup).
Keep shared project, style, and formatting rules in `AGENTS.md`; add rules here only
when they are specific to Claude Code. Both instruction files belong to the author
checkout and are removed from prepared student copies by `utils/setup_student.ps1`.

## Terminal commands and screen access

Use terminal commands only for Remus work. Never use screen capture or computer
control unless David explicitly requests that action or grants permission first.
This includes screenshot tools, desktop or browser UI automation, simulated mouse
or keyboard input, and capture or control helpers launched from the terminal.
A general request to inspect, test, or fix something does not authorize screen
capture or computer control. If either is needed, explain the proposed action
and wait for permission before starting it.

## Chat responses

Responses are read in the Claude Code panel inside VS Code, which has no MathJax or
KaTeX renderer, so LaTeX shows up as literal dollar signs and backslashes.

- Write math as plain text: `2^n`, `<=`, `sqrt(2)`. Do not use `$...$` or `$$...$$`.
- Chat replies follow the repository's American spelling and no-long-dash rules.
- Use plain paragraphs for standard explanations rather than scattered bullets or bold.

## Git commits

Commit messages must carry no trace of SI assistance. Write the message as the
repository owner would have written it.

- Do not add a `Co-Authored-By: Claude ...` trailer, or any similar
  "co-authored by Claude" line.
- Do not add a generated-by footer such as "Generated with Claude Code" or a robot
  emoji tag.
- Do not credit the work in the subject or body either: no "with Claude's help",
  "SI-assisted", "per Claude's suggestion", or equivalent.
- Do not use vague verbs like "Refactor...", "Optimize...", or "Enhance..." unless
  followed by granular specifics. Never write passive commit messages.

The ban is on **attribution**, not on the words themselves. When a change is genuinely
*about* Claude or SI tooling, name it plainly:

- Good: `Add anthropic.claude-code to the extension install scripts`
- Good: `Document the chat math convention in CLAUDE.md`
- Bad: `Update install scripts (generated with Claude Code)`

### Push straight to main

Commit to `main` and push `main` (David, 2026-11-04). Open no pull requests, and push
no other branch to GitHub, such as a `claude/...` session branch: every branch name
shows on the repository's home page.

### Commit as David

Before the first commit of a session, check the repository's Git author identity.
Use David's identity for the direct commits to `main`:

```powershell
git config user.name "David Biersach"
git config user.email "dbiersach@gmail.com"
```

Before a push, run `git log -1 --format='%an <%ae>%n%B'` and confirm the author is
David and the message holds no `Co-authored-by` line and no credit to an SI tool.
