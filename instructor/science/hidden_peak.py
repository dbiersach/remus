"""hidden_peak.py

Science AI: aim the engine's loop of "list the candidates, score each one, choose
the best" at a measurement problem.

A 10x10 sample hides a hot spot. Each measurement reads the value at one square,
from 0 far away up to 100 at the peak. The goal is to find the peak in as few
measurements as possible, the same choice an AI agent makes at a beamline at
Brookhaven's NSLS-II: where to measure next.

| Chess engine | Hidden peak |
| --- | --- |
| Legal moves | Squares not yet measured |
| Evaluation | Predicted value (exploit) plus distance from earlier readings (explore) |
| A one-move search: play the best-scoring move | Measure the best-scoring square now |
| Minimax: the opponent picks the minimum | Expectimax: average over a high or low reading |
| Games against Stockfish 1500 | A benchmark: average measurements over 100 hidden samples |

This file uses common Python modules and does not import the engine: it reuses
the engine's ideas, not its code.
"""

import math
import random

import matplotlib.pyplot as plt

SIZE = 10  # The sample is SIZE x SIZE squares
PEAK = 100  # The reading at the hot spot
EXPLORE_WEIGHT = 8  # Points of score per square of distance from the nearest reading


def make_grid(seed: int) -> list[list[int]]:
    """
    Make a hidden sample: a hot spot at a random square, fading with distance.

    Parameters
    ----------
    seed : int
        The random seed, so the same seed always makes the same sample.

    Returns
    -------
    list[list[int]]
        grid[row][col] holds the reading at that square, from 0 to PEAK.
    """
    chooser = random.Random(seed)
    peak_row = chooser.randrange(SIZE)
    peak_col = chooser.randrange(SIZE)
    width = chooser.uniform(1.0, 2.5)  # How far the heat spreads
    grid = []
    for row in range(SIZE):
        values = []
        for col in range(SIZE):
            distance = math.dist((row, col), (peak_row, peak_col))
            values.append(round(PEAK * math.exp(-((distance / width) ** 2))))
        grid.append(values)
    return grid


def measure(grid: list[list[int]], square: tuple[int, int]) -> int:
    """Return the reading at one square of the hidden sample."""
    row, col = square
    return grid[row][col]


def predicted_value(
    square: tuple[int, int], readings: dict[tuple[int, int], int]
) -> float:
    """
    Predict the reading at a square from the readings so far.

    Each reading counts more the closer it is (an inverse-distance average), so a
    square next to a hot reading is predicted to be hot.
    """
    total = 0.0
    weights = 0.0
    for measured, value in readings.items():
        distance = math.dist(square, measured)
        weight = 1 / distance**2
        total += weight * value
        weights += weight
    return total / weights


def candidate_squares(readings: dict[tuple[int, int], int]) -> list[tuple[int, int]]:
    """
    Return every square not yet measured, the "legal moves" of this problem.

    Parameters
    ----------
    readings : dict[tuple[int, int], int]
        The readings so far, keyed by (row, col).

    Returns
    -------
    list[tuple[int, int]]
        The unmeasured squares, row by row.
    """
    squares = []
    for row in range(SIZE):
        for col in range(SIZE):
            if (row, col) not in readings:
                squares.append((row, col))
    return squares


def score_candidate(
    square: tuple[int, int], readings: dict[tuple[int, int], int], explore_weight: float
) -> float:
    """
    Score a square as the next measurement, the "evaluation" of this problem.

    Parameters
    ----------
    square : tuple[int, int]
        The square to score.
    readings : dict[tuple[int, int], int]
        The readings so far.
    explore_weight : float
        How much to value distance from earlier readings. 0 means "only measure
        where it already looks hot"; a large value means "spread out and look
        everywhere".

    Returns
    -------
    float
        The predicted value (exploit) plus explore_weight times the distance to the
        nearest reading (explore).
    """
    nearest = math.inf  # Farther than any real square
    for measured in readings:
        nearest = min(nearest, math.dist(square, measured))
    return predicted_value(square, readings) + explore_weight * nearest


def choose_next_measurement(
    readings: dict[tuple[int, int], int], explore_weight: float = EXPLORE_WEIGHT
) -> tuple[int, int]:
    """
    Return the unmeasured square with the highest score.

    This is a one-move search: score every legal move and keep the best, the way
    an engine would if it looked only one move ahead.

    Parameters
    ----------
    readings : dict[tuple[int, int], int]
        The readings so far.
    explore_weight : float
        Handed on to score_candidate() for every square.

    Returns
    -------
    tuple[int, int]
        The candidate square with the highest score. When two squares tie, the
        one that comes first, row by row, wins.
    """
    candidates = candidate_squares(readings)
    best_square = candidates[0]
    best_score = score_candidate(best_square, readings, explore_weight)
    for square in candidates:
        score = score_candidate(square, readings, explore_weight)
        if score > best_score:
            best_square = square
            best_score = score
    return best_square


def random_strategy(readings: dict[tuple[int, int], int]) -> tuple[int, int]:
    """Measure a random unmeasured square, the baseline every AI must beat."""
    return random.choice(candidate_squares(readings))


def expectimax_choice(
    readings: dict[tuple[int, int], int], explore_weight: float = EXPLORE_WEIGHT
) -> tuple[int, int]:
    """
    Look two measurements ahead (teacher demo).

    For each candidate, imagine the reading coming back 20 points higher or 20
    points lower than predicted, each half the time. Value each outcome as the
    pretend reading plus the best score of the measurement after it, then average
    the two outcomes. Chess search takes the opponent's minimum here; nature is
    not an opponent, so this search averages. Averages give no cutoffs, which is
    why alpha-beta has nothing to prune.

    The benchmark shows this lookahead doing worse than the greedy AI: looking
    ahead only helps when the predictions are good, and a "plus or minus 20" guess
    is not. The NSLS-II agent uses a statistical model (a Gaussian process) that
    knows how uncertain each prediction is.
    """
    candidates = candidate_squares(readings)
    best_square = candidates[0]
    best_value = -math.inf
    for square in candidates:
        guess = predicted_value(square, readings)
        outcomes = []
        for reading in (min(PEAK, guess + 20), max(0, guess - 20)):
            # Pretend to measure on a copy, the way make_move() copies the board
            pretend = dict(readings)
            pretend[square] = round(reading)
            next_best = 0  # The score when no square is left to measure
            others = candidate_squares(pretend)
            if others:
                next_best = score_candidate(others[0], pretend, explore_weight)
            for other in others:
                next_best = max(
                    next_best, score_candidate(other, pretend, explore_weight)
                )
            outcomes.append(reading + next_best)
        value = sum(outcomes) / len(outcomes)
        if value > best_value:
            best_square, best_value = square, value
    return best_square


def measurements_to_find_peak(
    grid: list[list[int]], strategy, explore_weight: float | None = None
) -> int:
    """
    Run one strategy on one sample and count measurements until it reads PEAK.

    A strategy that takes an explore weight gets explore_weight; when it is None,
    the strategy gets only the readings.
    """
    center = (SIZE // 2, SIZE // 2)
    readings = {center: measure(grid, center)}  # Every strategy starts in the middle
    while PEAK not in readings.values():
        if explore_weight is None:
            square = strategy(readings)
        else:
            square = strategy(readings, explore_weight)
        readings[square] = measure(grid, square)
    return len(readings)


def benchmark(
    strategy, trials: int = 100, explore_weight: float | None = None
) -> float:
    """
    Return the average number of measurements a strategy needs over many samples.

    Parameters
    ----------
    strategy : callable
        Takes the readings so far and returns the next square to measure.
    trials : int
        How many hidden samples to try. Sample n always uses seed n, so every
        strategy faces the same samples.
    explore_weight : float | None
        The explore weight to hand the strategy, or None for a strategy that
        takes only the readings.

    Returns
    -------
    float
        The average measurements needed to find the peak.
    """
    random.seed(0)  # The random strategy makes the same choices every run
    total = 0
    for seed in range(trials):
        total += measurements_to_find_peak(make_grid(seed), strategy, explore_weight)
    return total / trials


def measurement_order(grid: list[list[int]], strategy) -> list[tuple[int, int]]:
    """Return the squares one strategy measures, in order, until it reads PEAK."""
    center = (SIZE // 2, SIZE // 2)
    readings = {center: measure(grid, center)}
    order: list[tuple[int, int]] = [center]
    while PEAK not in readings.values():
        square = strategy(readings)
        readings[square] = measure(grid, square)
        order.append(square)
    return order


def show_sample_graph(seed: int) -> None:
    """Draw one hidden sample with the squares each strategy measures numbered."""
    grid = make_grid(seed)
    random.seed(0)  # The random strategy makes the same choices every run
    strategies = (
        ("Random guessing", random_strategy),
        ("The AI", choose_next_measurement),
    )
    figure, axes = plt.subplots(1, 2, figsize=(12, 6.5))
    for ax, (name, strategy) in zip(axes, strategies, strict=True):
        order = measurement_order(grid, strategy)
        ax.imshow(grid, cmap="YlOrRd", vmin=0, vmax=PEAK)
        for number, (row, col) in enumerate(order, start=1):
            ax.text(col, row, str(number), ha="center", va="center")
        ax.set_xticks([])
        ax.set_yticks([])
        ax.set_title(f"{name}: {len(order)} measurements")
    figure.suptitle(
        "One hidden sample: each number is one measurement. Darker is hotter."
    )
    figure.tight_layout()
    print("Close the graph window to continue.")
    plt.show()


def show_benchmark_graph(results: list[tuple[str, float]]) -> None:
    """Draw the average measurements of each strategy, with the fewest in green."""
    fewest = results[0][1]
    for _, average in results:
        fewest = min(fewest, average)
    _, ax = plt.subplots(figsize=(12, 4.5))
    for row, (name, average) in enumerate(results):
        color = "tab:green" if average == fewest else "tab:gray"
        ax.barh(name, average, color=color)
        ax.text(average + 0.5, row, f"{average:.1f}", va="center")
    ax.invert_yaxis()  # The first strategy goes at the top
    ax.margins(x=0.1)  # Room for the number at the end of the longest bar
    ax.set_xlabel("Measurements needed to find the peak, on average")
    ax.set_title("Fewer measurements means a better strategy")
    plt.tight_layout()
    print("Close the graph window to continue.")
    plt.show()


def main():
    results = []
    average = benchmark(random_strategy)
    results.append(("Random guessing", average))
    print(f"Random guessing: {average:5.1f} measurements on average")
    for weight in (0, EXPLORE_WEIGHT, 40):
        average = benchmark(choose_next_measurement, explore_weight=weight)
        results.append((f"Greedy AI, weight {weight}", average))
        print(
            f"Greedy AI, explore weight {weight:2}: {average:5.1f} measurements on average"
        )
    print(
        "  (expected: every greedy AI well below random guessing, best near weight 8)"
    )
    average = benchmark(expectimax_choice, trials=10)
    results.append(("Expectimax (10 samples)", average))
    print(f"Expectimax AI (10 samples): {average:5.1f} measurements on average")
    print(
        "  (expected: better than random, worse than greedy: a crude model limits lookahead)"
    )
    show_sample_graph(3)  # Sample 3 hides its peak far from the center
    show_benchmark_graph(results)


if __name__ == "__main__":
    main()
