#[macro_use]
extern crate smallvec;

use aoc_runner_derive::aoc_lib;

pub mod solutions {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
}
mod util {
    pub mod parsers;
}

aoc_lib! { year = 2019 }
