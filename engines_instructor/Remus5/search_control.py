"""search_control.py

Read UCI input while the search works, and start the move clock on ponderhit.
Only the input reader runs in another thread; one thread owns all chess state.
"""

import json
import os
import sys
import time
from collections import deque
from pathlib import Path
from queue import Empty, Queue
from threading import Thread

SHALLOW_DEPTH = 8  # Below this completed depth, a hit deserves another look
SCORE_DROP = 35  # Centipawns lost between mature completed iterations
type PonderCredit = tuple[float, int, int | None, bool, bool]


class SearchControl:
    """Keep commands, cancellation, and the clock for one active search."""

    def __init__(self) -> None:
        self.diagnostics = False
        self.log_failed = False
        self.session_started = time.perf_counter()
        self.log_path = (
            Path(__file__).parent
            / "ponder_logs"
            / (f"ponder_{os.getpid()}_{time.time_ns()}.jsonl")
        )
        self.search_id = -1
        self.commands: Queue[str] = Queue()
        self.pending: deque[str] = deque()
        self.begin("normal", float("inf"))
        Thread(target=self.read_input, daemon=True).start()

    def log(self, event: str, **fields: str | float | bool | None) -> None:
        """Append boundary events to a process-specific file, never inside node search."""
        if not getattr(self, "diagnostics", False) or self.log_failed:
            return
        record = {
            "event": event,
            "pid": os.getpid(),
            "search": self.search_id,
            "unix_ms": time.time_ns() // 1_000_000,
            "elapsed_ms": (time.perf_counter() - self.session_started) * 1000,
            "cpu_ms": time.process_time() * 1000,
            **fields,
        }
        try:
            self.log_path.parent.mkdir(exist_ok=True)
            with self.log_path.open("a", encoding="utf-8") as file:
                file.write(json.dumps(record, allow_nan=False) + "\n")
        except OSError as error:
            self.log_failed = True
            print(f"info string PonderDiagnostics log unavailable: {error}", flush=True)

    def read_input(self) -> None:
        """Queue complete input lines and treat a closed pipe as quit."""
        for line in sys.stdin:
            self.commands.put(line)
        self.commands.put("quit")

    def next_command(self) -> str:
        """Return a deferred command before waiting for another input line."""
        if self.pending:
            return self.pending.popleft()
        return self.commands.get()

    def begin(self, mode: str, longest: float) -> None:
        """Set the search mode and its maximum time after ponderhit."""
        self.search_id = getattr(self, "search_id", -1) + 1
        self.mode = mode
        self.stopped = False
        self.stop_received = False
        self.credit_depth = 1
        self.hit_kind = "none"
        self.started = time.perf_counter()
        self.longest = longest
        self.search_started = self.started
        self.was_ponder = mode == "ponder"
        self.hit_time: float | None = None
        self.nodes = 0
        self.completed_depth = 0
        self.hit_nodes = 0
        self.hit_depth = 0
        self.soft_limit = float("inf")
        self.can_reinvest = False
        self.source = "native"
        self.score: int | None = None
        self.score_depth = 0
        self.deteriorating = False
        self.unstable = False
        self.uncertain_hit = False

    def handle(self, line: str) -> None:
        """Answer live commands or defer a command that replaces the search."""
        self.log("control", command=line.strip())
        words = line.split()
        if not words:
            return
        if words[0] == "isready":
            print("readyok", flush=True)
        elif words[0] == "ponderhit":
            if self.mode == "ponder":
                self.mode = "normal"
                self.started = time.perf_counter()
                self.hit_time = self.started
                self.hit_kind = "uci"
                self.hit_nodes = self.nodes
                self.hit_depth = self.completed_depth
                self.uncertain_hit = (
                    self.hit_depth < SHALLOW_DEPTH
                    or self.deteriorating
                    or self.unstable
                )
                self.log("hit", kind="uci", depth=self.hit_depth, nodes=self.hit_nodes)
        elif words[0] == "stop":
            self.stopped = True
            self.stop_received = True
        elif words[0] in ("position", "go", "setoption", "ucinewgame", "quit"):
            self.pending.append(line)
            self.stopped = True

    def poll(self, nodes: int = 0) -> bool:
        """Process available commands and report cancellation or a time limit."""
        self.nodes = nodes
        while not self.stopped:
            try:
                self.handle(self.commands.get_nowait())
            except Empty:
                break
        if self.stopped:
            return True
        if self.mode != "normal":
            return False
        if time.perf_counter() >= self.started + self.longest:
            return True
        # A hit can use a completed iteration without waiting for a deeper one.
        return (
            self.was_ponder
            and self.completed_depth >= self.credit_depth
            and time.perf_counter() >= self.search_started + self.soft_limit
            and self.hit_finished()
        )

    def hit_minimum(self) -> float:
        """Reserve a quarter of the adjusted plan after a hit, within the hard limit."""
        return min(self.soft_limit / 2, self.longest / 4)

    def reinvest(self) -> bool:
        """Use extra hit time with a clock reserve and an uncertain result."""
        return self.can_reinvest and (
            self.uncertain_hit or self.deteriorating or self.unstable
        )

    def hit_finished(self) -> bool:
        """Aim for another depth, bounded by the plan and half the hard budget."""
        if not self.reinvest():
            return self.elapsed() >= self.hit_minimum()
        minimum = min(self.soft_limit, self.longest / 2)
        limit = min(2 * self.soft_limit, self.longest / 2)
        return self.elapsed() >= minimum and (
            self.completed_depth > self.hit_depth or self.elapsed() >= limit
        )

    def soft_finished(self, target: float) -> bool:
        """Apply the soft limit without bypassing the post-hit thinking allowance."""
        return self.elapsed() > target and (
            not self.was_ponder
            or (self.completed_depth >= self.credit_depth and self.hit_finished())
        )

    def ponder_credit(self) -> PonderCredit | None:
        """Remember completed ponder work only after an explicit stop without a hit."""
        if (
            self.was_ponder
            and self.hit_time is None
            and self.stop_received
            and self.source == "native"
        ):
            return (
                time.perf_counter() - self.search_started,
                self.completed_depth,
                self.score,
                self.deteriorating,
                self.unstable,
            )
        return None

    def credit_ponder(
        self,
        seconds: float,
        depth: int,
        score: int | None = None,
        deteriorating: bool = False,
        unstable: bool = False,
    ) -> None:
        """Credit a matching stopped search after recovering its completed depth."""
        self.was_ponder = True
        self.credit_depth = max(1, depth)
        self.search_started = self.started - seconds
        self.hit_time = self.started
        self.hit_depth = depth
        self.hit_kind = "stop_go"
        self.score = score
        self.score_depth = depth
        self.deteriorating = deteriorating
        self.unstable = unstable
        self.uncertain_hit = depth < SHALLOW_DEPTH or deteriorating or unstable

    def wait(self) -> None:
        """Hold a finished ponder or infinite result until hit or cancellation."""
        while self.mode != "normal" and not self.stopped:
            self.handle(self.commands.get())

    def elapsed(self) -> float:
        """Return time on our clock, excluding time spent pondering."""
        if self.mode != "normal":
            return 0.0
        return time.perf_counter() - self.started

    def completed(
        self,
        depth: int,
        soft_limit: float,
        score: int | None = None,
        changed: bool = False,
    ) -> None:
        """Record usable search work and the target including ponder time."""
        self.completed_depth = depth
        self.soft_limit = soft_limit
        # Shallow replay after a Lucas hit must not overwrite its deeper score.
        if depth >= max(4, self.score_depth) and score is not None:
            if self.score is not None and self.score - score >= SCORE_DROP:
                self.deteriorating = True
            self.unstable = changed
            self.score = score
            self.score_depth = depth

    def report(self) -> None:
        """Save each search outcome and emit UCI diagnostics for ponder requests."""
        end = time.perf_counter()
        ponder_end = end if self.hit_time is None else self.hit_time
        outcome = "stopped" if self.hit_time is None else "hit"
        own_seconds = 0.0 if self.hit_time is None else end - self.hit_time
        own_ms = own_seconds * 1000 if self.was_ponder else self.elapsed() * 1000
        self.log(
            "search_end",
            outcome=outcome if self.was_ponder else "normal",
            hit_kind=self.hit_kind,
            ponder_ms=(ponder_end - self.search_started) * 1000
            if self.was_ponder
            else 0,
            own_ms=own_ms,
            hit_depth=self.hit_depth,
            hit_nodes=self.hit_nodes,
            final_depth=self.completed_depth,
            final_nodes=self.nodes,
            soft_target_ms=None
            if self.soft_limit == float("inf")
            else self.soft_limit * 1000,
            stopped=self.stopped,
            source=self.source,
            reinvest=self.was_ponder and self.source == "native" and self.reinvest(),
            shallow_hit=self.hit_depth < SHALLOW_DEPTH,
            deteriorating=self.deteriorating,
            unstable=self.unstable,
            hit_cap_ms=None
            if self.soft_limit == float("inf")
            else min(2 * self.soft_limit, self.longest / 2) * 1000,
        )
        if not self.was_ponder:
            return
        print(
            f"info string ponder outcome={outcome} "
            f"ponder_ms={(ponder_end - self.search_started) * 1000:.1f} "
            f"hit_depth={self.hit_depth} hit_nodes={self.hit_nodes} "
            f"final_depth={self.completed_depth} final_nodes={self.nodes} "
            f"own_ms={own_seconds * 1000:.1f} hit_kind={self.hit_kind}",
            flush=True,
        )
