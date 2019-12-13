use crate::util::intcode::{parse_intcode_text, Emulator, RunResult, Word};
use aoc_runner_derive::{aoc, aoc_generator};
use arraydeque::{ArrayDeque, Wrapping};
use arrayvec::ArrayVec;
use itertools::Itertools;
use nalgebra::Point2;
use nom::lib::std::collections::VecDeque;
use std::{
    collections::HashMap,
    error::Error,
    io::{stdout, Write},
    thread,
    time::Duration,
};
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};

type GeneratorOutput = Vec<Word>;
type PartInput = [Word];

#[aoc_generator(day13)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    parse_intcode_text(input)
}

#[aoc(day13, part1)]
pub fn part_1(input: &PartInput) -> usize {
    let mut emulator = Emulator::new(input.to_owned());
    let mut tile_map = HashMap::<Point2<i64>, i64>::new();

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
pub fn part_2(input: &PartInput) -> i64 {
    let mut stdin = async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut memory = input.to_owned();
    memory[0] = 2;
    let mut emulator = Emulator::new(memory);
    let mut tile_map = HashMap::<Point2<i64>, i64>::new();
    let mut score = 0;
    //    let mut inputs = ArrayDeque::<[i64; 1], Wrapping>::new();

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    let mut ball_position: Option<i64> = None;
    let mut paddle_position: Option<i64> = None;

    'game: loop {
        //        while let Some(Ok(key)) = stdin.next() {
        //            match key {
        //                Key::Left => {
        //                    inputs.push_back(-1);
        //                }
        //                Key::Right => {
        //                    inputs.push_back(1);
        //                }
        //                Key::Esc | Key::Ctrl('c') => {
        //                    break 'game;
        //                }
        //                _ => {}
        //            }
        //        }

        let mut run_game = || loop {
            match emulator.run() {
                RunResult::Output(val) => return Some(val),
                RunResult::InputRequest => {
                    if let (Some(ball_pos), Some(paddle_pos)) = (ball_position, paddle_position) {
                        emulator.push_input((ball_pos - paddle_pos).signum());
                    }
                }
                _ => return None,
            }
        };
        let x = match run_game() {
            Some(val) => val,
            None => break 'game,
        };
        let y = match run_game() {
            Some(val) => val,
            None => break 'game,
        };
        let tile = match run_game() {
            Some(val) => val,
            None => break 'game,
        };

        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();

        if let (-1, 0) = (x, y) {
            score = tile;
        } else {
            tile_map.insert(Point2::new(x, y), tile);
        }

        write!(stdout, "Score: {}\n", score).unwrap();

        let mut has_paddle = false;

        let rx = tile_map.keys().map(|p| p.x).minmax().into_option().unwrap();
        let ry = tile_map.keys().map(|p| p.y).minmax().into_option().unwrap();
        for y in ry.0..=ry.1 {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1, (y - ry.0 + 3) as u16)
            )
            .unwrap();
            for x in rx.0..=rx.1 {
                let char = match tile_map.get(&Point2::new(x, y)) {
                    None => " ",
                    Some(0) => " ",
                    Some(1) => "█",
                    Some(2) => "#",
                    Some(3) => {
                        paddle_position = Some(x);
                        "="
                    }
                    Some(4) => {
                        ball_position = Some(x);
                        "O"
                    }
                    _ => "?",
                };
                write!(stdout, "{}", char).unwrap();
            }
        }
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(2));
    }

    score
}
