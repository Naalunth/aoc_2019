//use aoc_runner_derive::aoc_main;

//aoc_main! { lib = aoc_naalunth_2019 }
use aoc_naalunth_2019::solutions::day07;
use bstr::ByteSlice;
use criterion::black_box;
use std::time::Instant;

fn main() {
    let input = include_bytes!("../input/2019/day7.txt");
    let generated_stuff = day07::generator(input.trim_end_with(|c| c == '\n')).unwrap();
    let start = Instant::now();
    for _ in 0..10000 {
        black_box(day07::part_1(black_box(&generated_stuff)));
    }
    let duration = Instant::now() - start;
    println!("Duration: {} ns", duration.as_nanos() / 10000)
}
