use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;
use ordered_float::OrderedFloat;
use smallvec::SmallVec;
use std::collections::BTreeMap;

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

fn fraction_order_key((x, y): (i16, i16)) -> f64 {
    -f64::atan2(x as f64, y as f64)
}

#[aoc(day10, part2)]
pub fn part_2(input: &PartInput) -> i16 {
    let base = find_base(input).0;

    let mut others = input
        .iter()
        .filter(|&coords| *coords != base)
        .map(|&(ox, oy)| (ox - base.0, oy - base.1))
        .map(|coords| (fraction_order_key(reduce_fraction(coords)), coords))
        .fold(
            BTreeMap::<OrderedFloat<f64>, SmallVec<[(i16, i16); 2]>>::new(),
            |mut map, (k, (x, y))| {
                map.entry(OrderedFloat::from(k))
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
