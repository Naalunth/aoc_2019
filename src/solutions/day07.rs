use crate::util::parsers::signed_number;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::lib::std::iter::successors;
use rayon::prelude::*;
use std::{
    error::Error,
    mem::replace,
    sync::mpsc::{channel, Receiver, Sender},
};

type GeneratorOutput = Vec<i64>;
type PartInput = [i64];

#[aoc_generator(day7)]
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

fn intcode(mem: &mut Vec<i64>, input: Receiver<i64>, output: Sender<i64>) {
    fn get_operand(mem: &mut Vec<i64>, address: usize, mode: u8) -> i64 {
        if mode == 0 {
            mem[mem[address] as usize]
        } else {
            mem[address]
        }
    }
    let mut pc = 0usize;
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
                let input_value = input.recv().unwrap();
                let write_address = mem[pc + 1] as usize;
                mem[write_address] = input_value;
                pc += 2;
            }
            4 => {
                let read_address = mem[pc + 1] as usize;
                output.send(mem[read_address]).unwrap();
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
}

#[aoc(day7, part1)]
pub fn part_1(original: &PartInput) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|params| {
            params.iter().fold(0, |acc, param| {
                let mut memory = original.to_owned();
                let (send_input, input) = channel();
                let (output, recv_output) = channel();
                send_input.send(*param).unwrap();
                send_input.send(acc).unwrap();
                intcode(&mut memory, input, output);
                recv_output.recv().unwrap()
            })
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part_2(original: &PartInput) -> i64 {
    (5..=9)
        .permutations(5)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|params| {
            let (first_input, input_a) = channel::<i64>();
            let mut last_output = None;
            let computer_connections = params
                .iter()
                .scan(
                    (Some(input_a), first_input.clone(), 0),
                    |(last_input, last_send_input, counter), param| {
                        let memory = original.to_owned();
                        let (output, recv_output) = channel();
                        let input = replace(last_input, Some(recv_output));
                        last_send_input.send(*param).unwrap();
                        if *counter == 4 {
                            last_output = replace(last_input, None);
                        }
                        *counter += 1;
                        replace(last_send_input, output.clone());
                        Some((memory, input.unwrap(), output))
                    },
                )
                .collect::<Vec<_>>();
            let last_output = last_output.unwrap();

            for (mut memory, input, output) in computer_connections.into_iter() {
                std::thread::spawn(move || {
                    intcode(&mut memory, input, output);
                });
            }

            successors(Some(0), |&x| {
                first_input.send(x).ok()?;
                last_output.recv().ok()
            })
            .last()
            .unwrap()
        })
        .max()
        .unwrap()
}
