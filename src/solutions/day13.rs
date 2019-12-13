use crate::util::intcode::{parse_intcode_text, Emulator, RunResult};
use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::Point2;
use std::{collections::HashMap, error::Error};

type Word = i32;
type GeneratorOutput = Vec<Word>;
type PartInput = [Word];

#[aoc_generator(day13)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    parse_intcode_text(input)
}

#[aoc(day13, part1)]
pub fn part_1(input: &PartInput) -> usize {
    let mut emulator = Emulator::<Word>::new(input.to_owned());
    let mut tile_map = HashMap::<Point2<Word>, Word>::new();

    loop {
        let x = match emulator.run() {
            RunResult::Output(val) => val,
            _ => break,
        };
        let y = match emulator.run() {
            RunResult::Output(val) => val,
            _ => break,
        };
        let tile = match emulator.run() {
            RunResult::Output(val) => val,
            _ => break,
        };
        tile_map.insert(Point2::new(x, y), tile);
    }

    tile_map.values().filter(|t| **t == 2).count()
}

#[aoc(day13, part2)]
pub fn part_2(input: &PartInput) -> Word {
    let mut memory = input.to_owned();
    memory[0] = 2;
    let mut emulator = Emulator::new(memory);

    let mut score = 0;
    let mut ball_position: Option<Word> = None;
    let mut paddle_position: Option<Word> = None;

    || -> Option<()> {
        loop {
            let mut run_game = || loop {
                match emulator.run() {
                    RunResult::Output(val) => break Some(val),
                    RunResult::InputRequest => match (ball_position, paddle_position) {
                        (Some(b), Some(p)) => emulator.push_input((b - p).signum()),
                        _ => emulator.push_input(0),
                    },
                    _ => break None,
                }
            };
            let x = run_game()?;
            let y = run_game()?;
            let tile = run_game()?;

            match (x, y, tile) {
                (-1, 0, t) => score = t,
                (x, _, 3) => paddle_position = Some(x),
                (x, _, 4) => ball_position = Some(x),
                _ => {}
            }
        }
    }();

    score
}
