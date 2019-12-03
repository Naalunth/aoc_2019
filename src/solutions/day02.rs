use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use std::error::Error;

type GeneratorOutput = Vec<u32>;
type PartInput = [u32];

#[aoc_generator(day2)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b","), unsigned_number::<u32>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

fn intcode(mem: &mut Vec<u32>) {
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
}

#[aoc(day2, part1)]
pub fn part_1(input: &PartInput) -> u32 {
    let mut memory = input.to_vec();
    memory[1] = 12;
    memory[2] = 2;
    intcode(&mut memory);
    memory[0]
}

#[aoc(day2, part2)]
pub fn part_2(input: &PartInput) -> u32 {
    let mut memory = input.to_vec();
    for (noun, verb) in iproduct!(0..100, 0..100) {
        memory[1] = noun;
        memory[2] = verb;
        intcode(&mut memory);
        if memory[0] == 19690720 {
            return 100 * noun + verb;
        }
        memory.copy_from_slice(input);
    }
    panic!("no verb-noun combo found")
}

#[aoc(day2, part2, cheating)]
pub fn part_2_cheat(input: &PartInput) -> u32 {
    let mut memory = input.to_vec();
    memory[1] = 0;
    memory[2] = 0;
    intcode(&mut memory);
    let c = memory[0];

    memory.copy_from_slice(input);
    memory[1] = 1;
    memory[2] = 0;
    intcode(&mut memory);
    let a = memory[0] - c;

    let t = 19690720 - c;
    t % a + t / a * 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode() {
        let mut memory = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        intcode(&mut memory);
        assert_eq!(&memory, &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);

        let mut memory = vec![1, 0, 0, 0, 99];
        intcode(&mut memory);
        assert_eq!(&memory, &[2, 0, 0, 0, 99]);

        let mut memory = vec![2, 3, 0, 3, 99];
        intcode(&mut memory);
        assert_eq!(&memory, &[2, 3, 0, 6, 99]);

        let mut memory = vec![2, 4, 4, 5, 99, 0];
        intcode(&mut memory);
        assert_eq!(&memory, &[2, 4, 4, 5, 99, 9801]);

        let mut memory = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        intcode(&mut memory);
        assert_eq!(&memory, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
