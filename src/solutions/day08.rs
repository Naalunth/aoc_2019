use crate::util::parsers::unsigned_number;
use aoc_runner_derive::{aoc, aoc_generator};
use std::error::Error;
use std::io::Write;

type GeneratorOutput = Vec<u32>;
type PartInput = [u32];

#[aoc_generator(day8)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::bytes::complete::take;
    use nom::combinator::all_consuming;
    use nom::combinator::map_parser;
    use nom::multi::many0;
    Ok(
        all_consuming(many0(map_parser(take(1usize), unsigned_number::<u32>)))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

#[aoc(day8, part1)]
pub fn part_1(input: &PartInput) -> u32 {
    input
        .chunks(150)
        .map(|layer| {
            let mut counts = [0u32; 3];
            for row in 0..6 {
                for col in 0..25 {
                    let pixel = layer[row * 25 + col];
                    counts[pixel as usize] += 1;
                }
            }
            (counts[0], (counts[1] * counts[2]))
        })
        .min_by_key(|(zeroes, _)| *zeroes)
        .map(|(_, score)| score)
        .unwrap()
}

#[aoc(day8, part2)]
pub fn part_2(input: &PartInput) -> String {
    let mut render_target = [0; 150];
    input.chunks(150).rev().for_each(|layer| {
        for row in 0..6 {
            for col in 0..25 {
                match layer[row * 25 + col] {
                    pixel @ (0..=1) => render_target[row * 25 + col] = pixel,
                    _ => {}
                }
            }
        }
    });

    let mut output_image = Vec::<u8>::new();
    write!(&mut output_image, "\n").unwrap();
    for row in 0..6 {
        for col in 0..25 {
            write!(
                &mut output_image,
                "{}",
                match render_target[row * 25 + col] {
                    1 => "#",
                    _ => " ",
                }
            )
            .unwrap();
        }
        write!(&mut output_image, "\n").unwrap();
    }
    String::from_utf8(output_image).unwrap()
}
