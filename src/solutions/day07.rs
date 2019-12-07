use crate::util::intcode::{parse_intcode_text, Emulator, RunResult, Word};
use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::{error::Error, mem::replace};

type GeneratorOutput = Vec<Word>;
type PartInput = [Word];

#[aoc_generator(day7)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    parse_intcode_text(input)
}

#[aoc(day7, part1)]
pub fn part_1(original: &PartInput) -> Word {
    let mut memory = Some(original.to_owned());
    (0..=4)
        .permutations(5)
        .map(|params| {
            params.iter().fold(0, |acc, &param| {
                let mut emulator = Emulator::new(replace(&mut memory, None).unwrap());
                emulator.extend_input([param, acc].iter().cloned());
                if let RunResult::Output(output) = unsafe { emulator.run_unchecked() } {
                    let mut mem = emulator.into_memory();
                    mem.copy_from_slice(original);
                    replace(&mut memory, Some(mem));
                    output
                } else {
                    panic!()
                }
            })
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part_2(original: &PartInput) -> Word {
    let mut memories = (0..=4)
        .map(|_| Some(original.to_owned()))
        .collect::<ArrayVec<[Option<Vec<Word>>; 5]>>();
    (5..=9)
        .permutations(5)
        .map(|params| {
            let mut emulators: Vec<Emulator> = params
                .iter()
                .zip(memories.iter_mut())
                .map(|(&param, memory)| {
                    let mut emulator = Emulator::new(replace(memory, None).unwrap());
                    emulator.push_input(param);
                    emulator
                })
                .collect::<Vec<_>>();

            let mut val = 0;
            'feedback: loop {
                for emulator in emulators.iter_mut() {
                    emulator.push_input(val);
                    val = 'run: loop {
                        match unsafe { emulator.run_unchecked() } {
                            RunResult::Halt => break 'feedback,
                            RunResult::InputRequest => panic!(),
                            RunResult::Output(output) => break 'run output,
                        }
                    }
                }
            }
            for (emulator, memory) in emulators.into_iter().zip(memories.iter_mut()) {
                let mut mem = emulator.into_memory();
                mem.copy_from_slice(original);
                replace(memory, Some(mem));
            }
            val
        })
        .max()
        .unwrap()
}
