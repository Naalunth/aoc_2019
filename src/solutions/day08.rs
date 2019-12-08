use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{zip, Itertools};

type GeneratorOutput = Vec<u8>;
type PartInput = [u8];

#[aoc_generator(day8)]
pub fn generator(input: &[u8]) -> GeneratorOutput {
    input.iter().map(|b| b - b'0').collect()
}

const ROW_COUNT: usize = 6;
const COLUMN_COUNT: usize = 25;
const PIXEL_COUNT: usize = ROW_COUNT * COLUMN_COUNT;

#[aoc(day8, part1)]
pub fn part_1(input: &PartInput) -> u32 {
    input
        .chunks_exact(PIXEL_COUNT)
        .map(|layer| {
            let mut counts = [0u32; 3];
            for &pixel in layer {
                counts[pixel as usize] += 1;
            }
            counts
        })
        .min()
        .map(|[_, c1, c2]| c1 * c2)
        .unwrap()
}

#[aoc(day8, part2)]
pub fn part_2(input: &PartInput) -> String {
    let mut render_target = [0; PIXEL_COUNT];
    for layer in input.chunks_exact(PIXEL_COUNT).rev() {
        for (&pixel, rt_pixel) in zip(layer, render_target.iter_mut()) {
            if pixel < 2 {
                *rt_pixel = pixel;
            }
        }
    }

    format!(
        "\n{}",
        render_target
            .chunks_exact(COLUMN_COUNT)
            .map(|row| {
                row.iter()
                    .map(|&pixel| match pixel {
                        1 => "#",
                        _ => " ",
                    })
                    .join("")
            })
            .format("\n")
    )
}
