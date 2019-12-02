use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

type GeneratorOutput = Vec<u64>;
type PartInput = [u64];

#[aoc_generator(day2)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b","), unsigned_number::<u64>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

pub fn run_intcode(program: &[u64], noun: u64, verb: u64) -> u64 {
    let mut mem = program.to_vec();
    mem[1] = noun;
    mem[2] = verb;

    let mut pc = 0usize;
    'main_loop: loop {
        let op = mem[pc];
        match op {
            1 => {
                let val = mem[mem[pc + 1] as usize] + mem[mem[pc + 2] as usize];
                let address = mem[pc + 3] as usize;
                mem[address] = val;
                pc += 4;
            }
            2 => {
                let val = mem[mem[pc + 1] as usize] * mem[mem[pc + 2] as usize];
                let address = mem[pc + 3] as usize;
                mem[address] = val;
                pc += 4;
            }
            99 => {
                break 'main_loop;
            }
            _ => panic!("unexpected op"),
        }
    }

    mem[0]
}

#[aoc(day2, part1)]
pub fn part_1(input: &PartInput) -> u64 {
    run_intcode(input, 12, 2)
}

#[aoc(day2, part2)]
pub fn part_2(input: &PartInput) -> u64 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_intcode(input, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("no valid inputs found")
}
