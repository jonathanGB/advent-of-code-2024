# Part 2 benchmarks

The first time I ran the solution for part 2, I timed it to take an average of 15s. I then re-ran in release mode (`cargo run --release day6 part2`), and this time it took about 1s. This shows how much of a performance improvement can be had just by not running the debug binary.

I then considered whether distributing the work of finding potential obstruction sites across multiple threads would further improve performance. We had to patrol over 5k potential sites, so it was definitely promising.

| Implementation           | Runtime   |
|--------------------------|-----------|
| Debug w/out parallelism  | 15,000 ms |
| Release w/out paralleism |  1,138 ms |
| Release w/ parallelism   |    138 ms |

So we effectively got an extra 10x improvement by dividing the work across threads! 