use crate::util::parsers::signed_number;
use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;

type GeneratorOutput = Vec<i64>;
type PartInput = [i64];

#[aoc_generator(day5)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b","), signed_number::<i64>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

type Instruction = [u8; 4];

fn decode_instruction(mut n: i64) -> Instruction {
    let mut result = [0; 4];
    result[0] = (n % 100) as u8;
    n /= 100;
    let mut i = 1;
    loop {
        result[i] = (n % 10) as u8;
        if i == 3 {
            break result;
        }
        i += 1;
        n /= 10;
    }
}

fn intcode(mem: &mut Vec<i64>, mut input: impl Iterator<Item = i64>) -> Vec<i64> {
    fn get_operand(mem: &mut Vec<i64>, address: usize, mode: u8) -> i64 {
        if mode == 0 {
            mem[mem[address] as usize]
        } else {
            mem[address]
        }
    }
    let mut pc = 0usize;
    let mut output = Vec::new();
    'main_loop: loop {
        let instruction = mem[pc];
        let instruction = decode_instruction(instruction);
        match instruction[0] {
            1 => {
                let operand_0 = get_operand(mem, pc + 1, instruction[1]);
                let operand_1 = get_operand(mem, pc + 2, instruction[2]);
                let write_address = mem[pc + 3] as usize;
                mem[write_address] = operand_0 + operand_1;
                pc += 4;
            }
            2 => {
                let operand_0 = get_operand(mem, pc + 1, instruction[1]);
                let operand_1 = get_operand(mem, pc + 2, instruction[2]);
                let write_address = mem[pc + 3] as usize;
                mem[write_address] = operand_0 * operand_1;
                pc += 4;
            }
            3 => {
                let input_value = input.next().unwrap();
                let write_address = mem[pc + 1] as usize;
                mem[write_address] = input_value;
                pc += 2;
            }
            4 => {
                let read_address = mem[pc + 1] as usize;
                output.push(mem[read_address]);
                pc += 2;
            }
            5 => {
                let operand_0 = get_operand(mem, pc + 1, instruction[1]);
                let operand_1 = get_operand(mem, pc + 2, instruction[2]);
                if operand_0 != 0 {
                    pc = operand_1 as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                let operand_0 = get_operand(mem, pc + 1, instruction[1]);
                let operand_1 = get_operand(mem, pc + 2, instruction[2]);
                if operand_0 == 0 {
                    pc = operand_1 as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                let operand_0 = get_operand(mem, pc + 1, instruction[1]);
                let operand_1 = get_operand(mem, pc + 2, instruction[2]);
                let write_address = mem[pc + 3] as usize;
                if operand_0 < operand_1 {
                    mem[write_address] = 1;
                } else {
                    mem[write_address] = 0;
                }
                pc += 4;
            }
            8 => {
                let operand_0 = get_operand(mem, pc + 1, instruction[1]);
                let operand_1 = get_operand(mem, pc + 2, instruction[2]);
                let write_address = mem[pc + 3] as usize;
                if operand_0 == operand_1 {
                    mem[write_address] = 1;
                } else {
                    mem[write_address] = 0;
                }
                pc += 4;
            }
            99 => {
                break 'main_loop;
            }
            _ => panic!("unexpected op"),
        }
    }
    output
}

#[aoc(day5, part1)]
pub fn part_1(input: &PartInput) -> i64 {
    let mut memory = input.to_owned();
    let output = intcode(&mut memory, [1].iter().cloned());
    *output.last().unwrap()
}

#[aoc(day5, part2)]
pub fn part_2(input: &PartInput) -> i64 {
    let mut memory = input.to_owned();
    let output = intcode(&mut memory, [5].iter().cloned());
    *output.last().unwrap()
}
