use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
pub struct Vector {
    dir: u8,
    len: i32,
}
#[derive(Debug)]
pub struct Wire {
    vectors: Vec<Vector>,
}

type GeneratorOutput = Vec<Wire>;
type PartInput = [Wire];

fn parse_vectors(input: &[u8]) -> IResult<&[u8], Vector> {
    use nom::bytes::complete::take;
    let (input, dir) = take(1usize)(input)?;
    let (input, len) = unsigned_number(input)?;
    Ok((input, Vector { dir: dir[0], len }))
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
        map(separated_list(tag(b","), parse_vectors), |vectors| Wire {
            vectors,
        }),
    ))(input)
    .map_err(|err| format!("Parser error: {:x?}", err))?
    .1)
}

impl Wire {
    fn points(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        self.vectors
            .iter()
            .flat_map(|vector| {
                let dir = vector.dir;
                (0..vector.len).map(move |_| dir)
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

#[derive(Debug, Clone)]
enum Segment {
    X { x: (i32, i32), y: i32 },
    Y { x: i32, y: (i32, i32) },
}

impl Wire {
    fn segments(&self) -> impl Iterator<Item = Segment> + '_ {
        self.vectors.iter().scan((0i32, 0i32), |point, vector| {
            Some(match vector.dir {
                b'R' => {
                    let start = point.0;
                    point.0 += vector.len;
                    Segment::X {
                        x: (start + 1, point.0),
                        y: point.1,
                    }
                }
                b'L' => {
                    let start = point.0;
                    point.0 -= vector.len;
                    Segment::X {
                        x: (point.0, start - 1),
                        y: point.1,
                    }
                }
                b'U' => {
                    let start = point.1;
                    point.1 += vector.len;
                    Segment::Y {
                        x: point.0,
                        y: (start + 1, point.1),
                    }
                }
                b'D' => {
                    let start = point.1;
                    point.1 -= vector.len;
                    Segment::Y {
                        x: point.0,
                        y: (point.1, start - 1),
                    }
                }
                _ => panic!(),
            })
        })
    }
}

impl Segment {
    fn intersection(&self, other: &Segment) -> Vec<(i32, i32)> {
        use Segment::*;
        match self {
            X { x, y } => match other {
                X {
                    x: other_x,
                    y: other_y,
                } => {
                    if *y == *other_y {
                        (x.0.max(other_x.0)..=x.1.min(other_x.1))
                            .map(|x| (x, *y))
                            .collect()
                    } else {
                        vec![]
                    }
                }
                Y {
                    x: other_x,
                    y: other_y,
                } => intersect_different_axes(*x, *other_x, *other_y, *y)
                    .into_iter()
                    .collect(),
            },
            Y { x, y } => match other {
                X {
                    x: other_x,
                    y: other_y,
                } => intersect_different_axes(*other_x, *x, *y, *other_y)
                    .into_iter()
                    .collect(),
                Y {
                    x: other_x,
                    y: other_y,
                } => {
                    if *x == *other_x {
                        (y.0.max(other_y.0)..=y.1.min(other_y.1))
                            .map(|y| (*x, y))
                            .collect()
                    } else {
                        vec![]
                    }
                }
            },
        }
    }
}

fn intersect_different_axes(
    x_range: (i32, i32),
    x_point: i32,
    y_range: (i32, i32),
    y_point: i32,
) -> Option<(i32, i32)> {
    if x_range.0 <= x_point && x_point <= x_range.1 && y_range.0 <= y_point && y_point <= y_range.1
    {
        Some((x_point, y_point))
    } else {
        None
    }
}

#[aoc(day3, part1, segments)]
pub fn part_1_segments(input: &PartInput) -> i32 {
    let segments = input[0].segments().collect::<Vec<_>>();
    input[1]
        .segments()
        .flat_map(|segment| {
            segments
                .iter()
                .flat_map(move |other| segment.intersection(other).into_iter())
        })
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = generator(
            b"R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
            U62,R66,U55,R34,D71,R55,D58,R83",
        )
        .unwrap();
        assert_eq!(part_1(&input), 159);
        let input = generator(
            b"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .unwrap();
        assert_eq!(part_1(&input), 135);
    }

    #[test]
    fn test_part_2() {
        let input = generator(
            b"R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
            U62,R66,U55,R34,D71,R55,D58,R83",
        )
        .unwrap();
        assert_eq!(part_2(&input), 610);
        let input = generator(
            b"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
            U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .unwrap();
        assert_eq!(part_2(&input), 410);
    }
}
