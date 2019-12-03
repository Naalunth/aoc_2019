use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
pub struct Line {
    dir: u8,
    len: u64,
}

type GeneratorOutput = Vec<Vec<Line>>;
type PartInput = [Vec<Line>];

fn parse_line(input: &[u8]) -> IResult<&[u8], Line> {
    use nom::bytes::complete::take;
    let (input, dir) = take(1usize)(input)?;
    let (input, len) = unsigned_number(input)?;
    Ok((input, Line { dir: dir[0], len }))
}
#[aoc_generator(day3)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(all_consuming(separated_list(
        tag(b"\n"),
        separated_list(tag(b","), parse_line),
    ))(input)
    .map_err(|err| format!("Parser error: {:x?}", err))?
    .1)
}

#[aoc(day3, part1)]
pub fn part_1(input: &PartInput) -> i32 {
    let mut current_point = (0i32, 0i32);
    let mut current_trace = HashSet::new();
    for line in &input[0] {
        for _ in 0..line.len {
            match line.dir {
                b'U' => current_point.1 += 1,
                b'D' => current_point.1 -= 1,
                b'L' => current_point.0 -= 1,
                b'R' => current_point.0 += 1,
                _ => panic!(),
            }
            current_trace.insert(current_point);
        }
    }
    let mut current_point = (0i32, 0i32);
    let mut intersections = vec![];
    for line in &input[1] {
        for _ in 0..line.len {
            match line.dir {
                b'U' => current_point.1 += 1,
                b'D' => current_point.1 -= 1,
                b'L' => current_point.0 -= 1,
                b'R' => current_point.0 += 1,
                _ => panic!(),
            }
            if current_trace.contains(&current_point) {
                intersections.push(current_point);
            }
        }
    }

    intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn part_2(input: &PartInput) -> i32 {
    let mut current_point = (0i32, 0i32);
    let mut current_distance = 0;
    let mut first_trace = HashMap::<(i32, i32), i32>::new();
    for line in &input[0] {
        for _ in 0..line.len {
            match line.dir {
                b'U' => current_point.1 += 1,
                b'D' => current_point.1 -= 1,
                b'L' => current_point.0 -= 1,
                b'R' => current_point.0 += 1,
                _ => panic!(),
            }
            current_distance += 1;
            first_trace.insert(current_point, current_distance);
        }
    }
    let mut current_point = (0i32, 0i32);
    let mut current_distance = 0;
    let mut intersections = vec![];
    for line in &input[1] {
        for _ in 0..line.len {
            match line.dir {
                b'U' => current_point.1 += 1,
                b'D' => current_point.1 -= 1,
                b'L' => current_point.0 -= 1,
                b'R' => current_point.0 += 1,
                _ => panic!(),
            }
            current_distance += 1;
            if let Some(other_distance) = first_trace.get(&current_point) {
                intersections.push((current_point, current_distance + *other_distance));
            }
        }
    }

    intersections
        .iter()
        .map(|(_, distance)| *distance)
        .min()
        .unwrap()
}
