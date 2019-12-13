use crate::util::intcode::{parse_intcode_text, Emulator, RunResult};
use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

type Word = i128;
type GeneratorOutput = Vec<Word>;
type PartInput = [Word];

#[aoc_generator(day9)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    parse_intcode_text(input)
}

fn run_program(program: &[Word], id: Word) -> Word {
    let mut emulator = Emulator::new(program.to_owned());
    emulator.push_input(id);
    let mut result = 0;
    while let RunResult::Output(val) = emulator.run() {
        result = val;
    }
    result
}

#[aoc(day9, part1)]
pub fn part_1(input: &PartInput) -> Word {
    run_program(input, 1)
}

#[aoc(day9, part2)]
pub fn part_2(input: &PartInput) -> Word {
    run_program(input, 2)
}
