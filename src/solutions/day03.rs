use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
pub struct Segment {
    dir: u8,
    len: u64,
}
#[derive(Debug)]
pub struct Wire {
    segments: Vec<Segment>,
}

type GeneratorOutput = Vec<Wire>;
type PartInput = [Wire];

fn parse_segment(input: &[u8]) -> IResult<&[u8], Segment> {
    use nom::bytes::complete::take;
    let (input, dir) = take(1usize)(input)?;
    let (input, len) = unsigned_number(input)?;
    Ok((input, Segment { dir: dir[0], len }))
}
#[aoc_generator(day3)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{
        bytes::complete::tag,
        combinator::{all_consuming, map},
        multi::separated_list,
    };
    Ok(all_consuming(separated_list(
        tag(b"\n"),
        map(separated_list(tag(b","), parse_segment), |segments| Wire {
            segments,
        }),
    ))(input)
    .map_err(|err| format!("Parser error: {:x?}", err))?
    .1)
}

impl Wire {
    fn points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.segments
            .iter()
            .flat_map(|segment| {
                let dir = segment.dir;
                (0..segment.len).map(move |_| dir)
            })
            .scan((0i32, 0i32), |point, dir| {
                match dir {
                    b'R' => point.0 += 1,
                    b'L' => point.0 -= 1,
                    b'U' => point.1 += 1,
                    b'D' => point.1 -= 1,
                    _ => panic!(),
                }
                Some(*point)
            })
    }

    fn distances(&self) -> impl Iterator<Item = ((i32, i32), i32)> + '_ {
        self.points().scan(0i32, |distance, point| {
            *distance += 1;
            Some((point, *distance))
        })
    }
}

#[aoc(day3, part1)]
pub fn part_1(input: &PartInput) -> i32 {
    let first_wire_points = input[0].points().collect::<HashSet<_>>();
    input[1]
        .points()
        .filter(|point| first_wire_points.contains(point))
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn part_2(input: &PartInput) -> i32 {
    let first_wire_distances = input[0]
        .distances()
        .fold(HashMap::new(), |mut map, (k, v)| {
            map.entry(k).or_insert(v);
            map
        });

    input[1]
        .distances()
        .filter_map(|(point, distance)| {
            first_wire_distances
                .get(&point)
                .map(|other| *other + distance)
        })
        .min()
        .unwrap()
}
