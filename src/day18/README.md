# Regarding part 2

The implementation initially found the first byte that partitions the start/exit space linearly. To be precise, we added bytes one after the other as corrupted from the list of remaining bytes (i.e. not the ones that were already added in part 1 and already showed to find a path) until a partition was detected. Benchmarks showed this linear algorithm to take ~250ms.

I have since tweaked to use a binary search algorithm, which results with the given input in only 12 BFS to find a path rather than 1000s. Benchmarks show this logarithm algorithm to take 500µs.