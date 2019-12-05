use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;
use std::error::Error;

type GeneratorOutput = (Password, Password);
type PartInput = GeneratorOutput;

fn parse_password(input: &[u8]) -> IResult<&[u8], Password> {
    use nom::{bytes::complete::take, combinator::map};
    let mut digits = [0; 6];
    let mut input = input;
    for i in 0..6 {
        let (input_, digit) = map(take(1usize), |d: &[u8]| d[0] - b'0')(input)?;
        input = input_;
        digits[i] = digit;
    }
    Ok((input, Password { digits }))
}

#[aoc_generator(day4)]
pub fn generator(input: &[u8]) -> Result<GeneratorOutput, Box<dyn Error>> {
    use nom::{bytes::complete::tag, combinator::all_consuming, sequence::separated_pair};
    Ok(
        all_consuming(separated_pair(parse_password, tag(b"-"), parse_password))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Password {
    digits: [u8; 6],
}

impl From<u32> for Password {
    fn from(mut num: u32) -> Self {
        let mut digits = [0u8; 6];
        for i in (0..6).rev() {
            digits[i] = (num % 10) as u8;
            num /= 10;
        }
        Self { digits }
    }
}
impl From<Password> for usize {
    fn from(pw: Password) -> Self {
        let mut result = 0usize;
        for digit in &pw.digits {
            result = (result + *digit as usize) * 10;
        }
        result / 10
    }
}

impl Password {
    fn inc(&mut self) {
        for i in (0..6).rev() {
            if self.digits[i] < 9 {
                self.digits[i] += 1;
                break;
            } else {
                self.digits[i] = 0;
            }
        }
    }

    fn inc_to_next_monotonic_number(&mut self) {
        let mut lock = None;
        for i in 1..6 {
            if let Some(lock) = lock {
                self.digits[i] = lock;
            } else {
                if self.digits[i - 1] > self.digits[i] {
                    self.digits[i] = self.digits[i - 1];
                    lock = Some(self.digits[i - 1]);
                }
            }
        }
    }

    fn condition_1(&self) -> bool {
        for i in 1..6 {
            if self.digits[i] == self.digits[i - 1] {
                return true;
            }
        }
        false
    }

    fn condition_2(&self) -> bool {
        let mut count = 1;
        for i in 1..6 {
            if self.digits[i] == self.digits[i - 1] {
                count += 1;
            } else {
                if count == 2 {
                    return true;
                }
                count = 1;
            }
        }
        count == 2
    }
}

fn count_passwords(
    first: Password,
    last: Password,
    condition: impl Fn(&Password) -> bool,
) -> usize {
    let mut pw = first.clone();
    pw.inc_to_next_monotonic_number();
    let mut count = 0;
    while pw <= last {
        if condition(&pw) {
            count += 1;
        }
        if pw == last {
            break;
        }
        pw.inc();
        pw.inc_to_next_monotonic_number();
    }
    return count;
}

#[aoc(day4, part1)]
pub fn part_1(&(first, last): &PartInput) -> usize {
    count_passwords(first, last, Password::condition_1)
}

#[aoc(day4, part2)]
pub fn part_2(&(first, last): &PartInput) -> usize {
    count_passwords(first, last, Password::condition_2)
}

// stolen from Discord user ppraisethesun))))#6504
#[aoc(day4, part1, stolen_from_ppraisethesun)]
pub fn part1_ppraisethesun(input: &PartInput) -> usize {
    let min = input.0.into();
    let max = input.1.into();
    let min_ht = min / 100000;
    let max_ht = (max as f64 / 100000.0).floor() as usize;

    let mut count = 0;
    for a in min_ht..max_ht + 1 {
        for b in a..10 {
            for c in b..10 {
                for d in c..10 {
                    for e in d..10 {
                        for f in e..10 {
                            if (a == b || b == c || c == d || d == e || e == f)
                                && (100000 * a + 10000 * b + 1000 * c + 100 * d + 10 * e + f) <= max
                                && (100000 * a + 10000 * b + 1000 * c + 100 * d + 10 * e + f) >= min
                            {
                                //println!("{}{}{}{}{}{}", a,b,c,d,e,f);
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

// stolen from Discord user ppraisethesun))))#6504
#[aoc(day4, part2, stolen_from_ppraisethesun)]
pub fn part2_ppraisethesun(input: &PartInput) -> usize {
    let min = input.0.into();
    let max = input.1.into();
    let min_ht = min / 100000;
    let max_ht = (max as f64 / 100000.0).floor() as usize;

    let mut count = 0;
    for a in min_ht..max_ht + 1 {
        for b in a..10 {
            for c in b..10 {
                for d in c..10 {
                    for e in d..10 {
                        for f in e..10 {
                            if ((a == b && b != c)
                                || (a != b && b == c && c != d)
                                || (b != c && c == d && d != e)
                                || (c != d && d == e && e != f)
                                || (d != e && e == f))
                                && (100000 * a + 10000 * b + 1000 * c + 100 * d + 10 * e + f) <= max
                                && (100000 * a + 10000 * b + 1000 * c + 100 * d + 10 * e + f) >= min
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(Password::from(111111).condition_1());
        // the digits are always in increasing order the way my program operates
        // assert!(!Password::from(223450).condition_1());
        assert!(!Password::from(123789).condition_1());
    }

    #[test]
    fn test_part_2() {
        assert!(Password::from(112233).condition_2());
        assert!(!Password::from(123444).condition_2());
        assert!(Password::from(111122).condition_2());
    }
}
