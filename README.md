# Advent of Code 2019 solutions

[![pipeline status](https://gitlab.com/Naalunth/aoc_2019/badges/master/pipeline.svg)](https://gitlab.com/Naalunth/aoc_2019/commits/master)
[![coverage report](https://gitlab.com/Naalunth/aoc_2019/badges/master/coverage.svg)](https://gitlab.com/Naalunth/aoc_2019/commits/master)

## Build
Should be built using the latest nightly Rust release. Requires [cargo-aoc](https://github.com/gobanos/cargo-aoc).
```
cargo aoc
```


## Benchmarks
All benchmarks are run on a Ryzen 7 2700X.

|  Day |                                     Part 1 |                                                                 Part 2 |
| ---: | -----------------------------------------: | ---------------------------------------------------------------------: |
|    1 |                base: 70 ns<br>simd: 115 ns | base: 1.5 µs<br>iterative: 947 ns<br>recursive: 942 ns<br>simd: 484 ns |
|    2 |                                      95 ns |                                       base: 787 µs<br>cheating: 180 ns |
|    3 | base: 12.8 ms<br>line intersection: 584 µs |                                                                14.8 ms |
|    4 |                                    5.16 µs |                                                                6.65 µs |
|    5 |                                     735 ns |                                                                 820 ns |
|    6 |            base: 75 µs<br>recursive: 74 µs |                                         base: 48 µs<br>in place: 33 µs |
|    7 |                                     ~20 ms |                                                                 ~50 ms |
