use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;
use smallvec::SmallVec;
use std::{cmp::Ordering, collections::BTreeMap, hint::unreachable_unchecked};

type GeneratorOutput = Vec<(i16, i16)>;
type PartInput = [(i16, i16)];

#[aoc_generator(day10)]
pub fn generator(input: &[u8]) -> GeneratorOutput {
    input
        .split(|c| *c == b'\n')
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| match *c {
                b'#' => Some((x as i16, y as i16)),
                _ => None,
            })
        })
        .collect()
}

fn reduce_fraction((x, y): (i16, i16)) -> (i16, i16) {
    let gcd = x.gcd(&y);
    (x / gcd, y / gcd)
}

fn find_base(asteroids: &[(i16, i16)]) -> ((i16, i16), usize) {
    asteroids
        .iter()
        .map(|&(x, y)| {
            (
                (x, y),
                asteroids
                    .iter()
                    .filter(|&coords| *coords != (x, y))
                    .map(|&(ox, oy)| reduce_fraction((ox - x, oy - y)))
                    .unique()
                    .count(),
            )
        })
        .max_by_key(|&(_, count)| count)
        .unwrap()
}

#[aoc(day10, part1)]
pub fn part_1(input: &PartInput) -> usize {
    find_base(input).1
}

#[derive(Debug, Eq, PartialEq)]
struct Coordinate(i16, i16);
impl Coordinate {
    fn slope(&self) -> f32 {
        self.1 as f32 / self.0 as f32
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0.signum(), other.0.signum()) {
            (1, 1) | (-1, -1) => self.slope().partial_cmp(&other.slope()),
            (_, -1) => Some(Ordering::Less),
            (0, 0) => self.1.signum().partial_cmp(&other.1.signum()),
            (0, 1) => self.1.signum().partial_cmp(&0),
            _ => other.partial_cmp(self).map(|ord| ord.reverse()),
        }
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[aoc(day10, part2)]
pub fn part_2(input: &PartInput) -> i16 {
    let base = find_base(input).0;

    let mut others = input
        .iter()
        .filter(|&coords| *coords != base)
        .map(|&(ox, oy)| (ox - base.0, oy - base.1))
        .fold(
            BTreeMap::<Coordinate, SmallVec<[(i16, i16); 2]>>::new(),
            |mut map, (x, y)| {
                let (rx, ry) = reduce_fraction((x, y));
                map.entry(Coordinate(rx, ry))
                    .and_modify(|vec| {
                        if let Err(pos) = vec.binary_search_by_key(&x.gcd(&y), |(x, y)| -x.gcd(y)) {
                            vec.insert(pos, (x, y))
                        }
                    })
                    .or_insert_with(|| smallvec![(x, y)]);
                map
            },
        );

    let mut destruction_counter = 0;
    loop {
        for queue in others.values_mut() {
            if let Some((x, y)) = queue.pop() {
                destruction_counter += 1;
                if destruction_counter == 200 {
                    return 100 * (x + base.0) + (y + base.1);
                }
            }
        }
    }
}
