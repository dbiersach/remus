# Minimax, Negamax, and Alpha-Beta: Video Resources

These videos illustrate game trees, evaluation, and alpha-beta cutoffs for a chess-engine programming workshop. A game tree branches into possible moves and replies. Minimax chooses a move assuming the opponent also chooses the best reply; alpha-beta skips branches that cannot change that choice.

## Top recommendations

### 1. Sebastian Lague: Algorithms Explained: minimax and alpha-beta pruning (best overall)

**Video:** [Watch on YouTube](https://www.youtube.com/watch?v=l-hh51ncgDI) · About 11 minutes

Use this short animation to introduce minimax and alpha-beta pruning. It shows cutoffs after explaining tree evaluation, and the linked pseudocode gives students a reference for implementing the search in Python.

### 2. Sebastian Lague: Coding Adventure: Chess

**Video:** [Watch on YouTube from Search and Evaluation (12:11)](https://www.youtube.com/watch?v=U4ogK0MIzqk&t=731s) · About 29 minutes

Use this video to connect tree search to a working chess engine. It demonstrates move generation, searching, evaluation, and later transposition tables. The **12:11** chapter begins the search-and-evaluation discussion. Use it after students understand smaller abstract game trees.

### 3. Gate Smashers: Lec-25: Alpha Beta Pruning in Hindi with Example

**Video:** [Watch on YouTube](https://www.youtube.com/watch?v=dEs_kbvu_0s) · **Language: Hindi**

A detailed worked example for students who need to see exactly when alpha and beta are updated. The walkthrough follows depth-first exploration, propagates values upward, and explains why branches can be skipped. Its presentation is closer to a conventional classroom lecture than Lague's animated explainer. The narration is in Hindi; confirm that students can follow it before assigning the video.

### 4. Patrick Winston / MIT OpenCourseWare: Lecture 6: Search: Games, Minimax, and Alpha-Beta

**Video:** [Watch on YouTube](https://www.youtube.com/watch?v=STjW3eH0Cik) · [MIT lecture page](https://ocw.mit.edu/courses/6-034-artificial-intelligence-fall-2010/resources/lecture-6-search-games-minimax-and-alpha-beta/)

Winston works through game decisions and cutoffs on a blackboard. Use the lecture after an introductory animation to examine the reasoning in more detail.

## Additional lectures

### 5. Spanning Tree: Minimax: How Computers Play Games (best alternative)

**Video:** [Watch on YouTube](https://www.youtube.com/watch?v=SLgZhpDsrfc) · About 13 minutes

The video progresses from basic minimax to pseudocode, game trees, alpha-beta pruning, and evaluation functions. Its chapter structure makes it easy to assign short classroom segments: **0:24** minimax, **5:12** pseudocode, **8:42** game trees, **10:28** alpha-beta pruning, and **12:19** evaluation functions.

### 6. Dave Churchill: COMP6980, Lecture 10: MiniMax + AlphaBeta Search

**Video:** [Watch on YouTube](https://www.youtube.com/watch?v=5ZwRWrNiRio) · [Lecture listing](https://www.classcentral.com/course/youtube-comp6980-intro-to-artificial-intelligence-lecture-10-minimax-alphabeta-search-320539) · About 1 hour 21 minutes

Use this lecture for explicit coverage of negamax: one recursive search handles both players by negating the returned score. The lecture covers minimax, **negamax at 32:13**, alpha-beta pruning at **36:42**, a worked alpha-beta example at **46:09**, recording the best move, and iterative deepening. The worked example shows how negamax simplifies a Python implementation.

## Which resource fits each algorithm?

| Topic | First choice | Why |
| --- | --- | --- |
| Minimax | [Spanning Tree](https://www.youtube.com/watch?v=SLgZhpDsrfc) | Clear progression from game rules to tree evaluation. |
| Alpha-beta pruning | [Sebastian Lague, Algorithms Explained](https://www.youtube.com/watch?v=l-hh51ncgDI) | Concise visual treatment of cutoffs. |
| Negamax | [Dave Churchill](https://www.youtube.com/watch?v=5ZwRWrNiRio&t=1933s) | Explicit explanation of the negamax formulation. |
| Applying search to chess | [Sebastian Lague, Coding Adventure: Chess](https://www.youtube.com/watch?v=U4ogK0MIzqk&t=731s) | Connects search to a functioning engine. |

## Interactive companion

[LK Forge: interactive minimax and alpha-beta visualization](https://lkforge.com/blog/minimax-alpha-beta-explained/) is a step-by-step game-tree visualization. Students can advance the search and compare ordinary minimax with alpha-beta pruning, seeing which branches are visited and which are skipped.

## Suggested classroom sequence

1. **Start with Spanning Tree:** Introduce MAX and MIN, leaf evaluations, recursion, and backing up scores.
2. **Follow with Sebastian Lague's Algorithms Explained:** Show how alpha-beta pruning preserves the minimax result while skipping unnecessary search.
3. **Use Dave Churchill for negamax:** Explain that the evaluation changes sign when the side to move changes.
4. **Finish with Coding Adventure: Chess:** Show how these ideas become the search function in a real chess engine.

For the negamax lesson, emphasize the identity

$$
\operatorname{negamax}(s,d)
=
\max_{m\in M(s)}\left[-\operatorname{negamax}(s_m,d-1)\right].
$$

Instead of separate maximizing and minimizing routines, the engine uses one recursive routine and negates the returned score. A practical implementation also needs base cases for terminal positions and the depth limit.

Selection notes use video descriptions, chapter structure, and educational references, rather than a complete review of every animation and narration. Preview the assigned segment before class.
