use crate::util::intcode::{parse_intcode_text, Emulator, RunResult, Word};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{Point2, Vector2};
use std::collections::HashSet;
use std::error::Error;

type GeneratorOutput = Vec<Word>;
type PartInput = [Word];

#[aoc_generator(day11)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    parse_intcode_text(input)
}

struct Robot {
    position: Point2<i64>,
    direction: Vector2<i64>,
}

impl Robot {
    fn turn_left(&mut self) {
        self.direction = Vector2::new(-self.direction.y, self.direction.x);
        self.position += self.direction;
    }
    fn turn_right(&mut self) {
        self.direction = Vector2::new(self.direction.y, -self.direction.x);
        self.position += self.direction;
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self {
            position: Point2::new(0, 0),
            direction: Vector2::new(0, 1),
        }
    }
}

#[aoc(day11, part1)]
pub fn part_1(input: &PartInput) -> usize {
    let mut robot = Robot::default();
    let mut grid = HashSet::<Point2<i64>>::new();
    let mut painted_tiles = HashSet::<Point2<i64>>::new();

    let mut emulator = Emulator::new(input.to_owned());

    loop {
        emulator.push_input(if grid.contains(&robot.position) { 1 } else { 0 });
        match emulator.run() {
            RunResult::Output(color) => match color {
                0 => {
                    grid.remove(&robot.position);
                }
                1 => {
                    grid.insert(robot.position.clone());
                    painted_tiles.insert(robot.position.clone());
                }
                _ => unreachable!(),
            },
            _ => break,
        }
        match emulator.run() {
            RunResult::Output(dir) => match dir {
                0 => robot.turn_left(),
                1 => robot.turn_right(),
                _ => unreachable!(),
            },
            _ => break,
        }
    }

    painted_tiles.len()
}

#[aoc(day11, part2)]
pub fn part_2(input: &PartInput) -> String {
    let mut robot = Robot::default();
    let mut grid = HashSet::<Point2<i64>>::new();
    grid.insert(Point2::new(0, 0));

    let mut emulator = Emulator::new(input.to_owned());

    loop {
        emulator.push_input(if grid.contains(&robot.position) { 1 } else { 0 });
        match emulator.run() {
            RunResult::Output(color) => match color {
                0 => {
                    grid.remove(&robot.position);
                }
                1 => {
                    grid.insert(robot.position.clone());
                }
                _ => unreachable!(),
            },
            _ => break,
        }
        match emulator.run() {
            RunResult::Output(dir) => match dir {
                0 => robot.turn_left(),
                1 => robot.turn_right(),
                _ => unreachable!(),
            },
            _ => break,
        }
    }

    let bx = grid.iter().map(|p| p.x).minmax().into_option().unwrap();
    let by = grid.iter().map(|p| p.y).minmax().into_option().unwrap();

    format!(
        "\n{}",
        (by.0..=by.1)
            .rev()
            .map(|y| {
                let grid = &grid;
                (bx.0..=bx.1)
                    .map(move |x| match grid.contains(&Point2::new(x, y)) {
                        true => "#",
                        false => " ",
                    })
                    .format("")
            })
            .format("\n")
    )
}
