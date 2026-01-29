use winnow::Result;
use winnow::ascii::dec_uint;
use winnow::combinator::{separated, separated_pair};
use winnow::prelude::*;

const _EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn parse_range(input: &mut &str) -> Result<(u64, u64)> {
    separated_pair(dec_uint, "-", dec_uint).parse_next(input)
}

fn parse_all_ranges(input: &mut &str) -> Result<Vec<(u64, u64)>> {
    separated(0.., parse_range, ",").parse_next(input)
}

pub fn resolve_part1(mut input: &str) -> u64 {
    let ranges = parse_all_ranges(&mut input).unwrap();
    let mut result = 0;
    for (start, end) in ranges {
        for num in start..=end {
            let num_as_string = num.to_string();
            let half = num_as_string.len() / 2;
            let (left, right) = num_as_string.split_at(half);
            if left == right {
                result += num;
            }
        }
    }
    result
}

fn is_repeated_sequence(num_as_string: &str) -> bool {
    let len = num_as_string.len();
    if len < 2 {
        return false;
    }
    let plen = len / 2;
    for p in 1..=plen {
        if len % p == 0 {
            let s = &num_as_string[0..p];
            if num_as_string
                .as_bytes()
                .chunks(p)
                .all(|chunk| chunk == s.as_bytes())
            {
                return true;
            }
        }
    }
    false
}

pub fn resolve_part2(mut input: &str) -> u64 {
    let ranges = parse_all_ranges(&mut input).unwrap();
    let mut result = 0;
    for (start, end) in ranges {
        for num in start..=end {
            let num_as_str = num.to_string();
            if is_repeated_sequence(num_as_str.as_str()) {
                result += num;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        let input_string = fs::read_to_string("input.txt").unwrap();
        let result = resolve_part1(&input_string);
        //let result = resolve_part1(_EXAMPLE);
        assert_eq!(result, 19128774598);
    }

    #[test]
    fn part2() {
        let input_string = fs::read_to_string("input.txt").unwrap();
        let result = resolve_part2(&input_string);
        //let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 21932258645);
    }
}
