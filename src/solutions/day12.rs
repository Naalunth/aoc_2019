use aoc_runner_derive::{aoc, aoc_generator};
use arrayvec::ArrayVec;
use itertools::{zip, Itertools};
use nalgebra::{Point3, Vector3};
use nom::IResult;
use num::Integer;
use std::error::Error;

type GeneratorOutput = Vec<Moon>;
type PartInput = [Moon];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Moon {
    pos: Point3<i32>,
    vel: Vector3<i32>,
}

pub fn parse_moon(input: &[u8]) -> IResult<&[u8], Moon> {
    use crate::util::parsers::signed_number;
    use nom::bytes::complete::tag;
    let (input, _) = tag("<x=")(input)?;
    let (input, x) = signed_number::<i32>(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = signed_number::<i32>(input)?;
    let (input, _) = tag(", z=")(input)?;
    let (input, z) = signed_number::<i32>(input)?;
    let (input, _) = tag(">")(input)?;
    Ok((
        input,
        Moon {
            pos: Point3::new(x, y, z),
            vel: Vector3::zeros(),
        },
    ))
}

#[aoc_generator(day12)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(all_consuming(separated_list(tag(b"\n"), parse_moon))(input)
        .map_err(|err| format!("Parser error: {:x?}", err))?
        .1)
}

#[aoc(day12, part1)]
pub fn part_1(input: &PartInput) -> i32 {
    let mut moons = input.to_vec();
    for _ in 0..1000 {
        for idx_a in 0..(moons.len() - 1) {
            for idx_b in (idx_a + 1)..moons.len() {
                let mut position_delta = moons[idx_b].pos - moons[idx_a].pos;
                position_delta.iter_mut().for_each(|c| *c = c.signum());
                moons[idx_a].vel += position_delta;
                moons[idx_b].vel -= position_delta;
            }
        }
        for Moon { pos, vel } in moons.iter_mut() {
            *pos += *vel;
        }
    }
    moons
        .iter()
        .map(|moon| {
            let pot = moon.pos.iter().map(|c| c.abs()).sum::<i32>();
            let kin = moon.vel.iter().map(|c| c.abs()).sum::<i32>();
            pot * kin
        })
        .sum()
}

fn cycle_length(moons: impl IntoIterator<Item = (i32, i32)>) -> u64 {
    let (m1, m2) = moons.into_iter().tee();
    let mut pos = m1
        .map(|m| m.0)
        .collect::<ArrayVec<[i32; 4]>>()
        .into_inner()
        .unwrap();
    let mut vel = m2
        .map(|m| m.1)
        .collect::<ArrayVec<[i32; 4]>>()
        .into_inner()
        .unwrap();
    let initial_state = (pos.clone(), vel.clone());
    for i in 1u64.. {
        for idx_a in 0..3 {
            for idx_b in (idx_a + 1)..4 {
                let delta = (pos[idx_b] - pos[idx_a]).signum();
                vel[idx_a] += delta;
                vel[idx_b] -= delta;
            }
        }
        for (p, v) in zip(&mut pos, &mut vel) {
            *p += *v;
        }
        if (pos, vel) == initial_state {
            return i;
        }
    }
    unreachable!()
}

#[aoc(day12, part2)]
pub fn part_2(input: &PartInput) -> u64 {
    let cx = cycle_length(input.iter().map(|m| (m.pos.x, m.vel.x)));
    let cy = cycle_length(input.iter().map(|m| (m.pos.y, m.vel.y)));
    let cz = cycle_length(input.iter().map(|m| (m.pos.z, m.vel.z)));
    cx.lcm(&cy).lcm(&cz)
}
