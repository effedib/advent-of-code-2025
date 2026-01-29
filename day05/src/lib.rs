use itertools::Itertools;
use std::collections::HashSet;

const _EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

pub fn resolve_part1(input_str: &str) -> usize {
    let (ranges_str, ids_str) = input_str.split_once("\n\n").expect("malformed input");
    let ranges: Vec<(u128, u128)> = ranges_str
        .lines()
        .filter_map(|line| {
            line.split_once("-")
                .and_then(|(start, end)| start.parse().ok().zip(end.parse().ok()))
        })
        .collect();

    let ids: HashSet<u128> = ids_str
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    ids.iter()
        .filter(|&id| {
            ranges
                .iter()
                .any(move |(start, end)| id >= start && id <= end)
        })
        .count()
}

pub fn resolve_part2(input_str: &str) -> u128 {
    let (ranges_str, _) = input_str.split_once("\n\n").expect("malformed input");
    ranges_str
        .lines()
        .filter_map(|line| {
            line.split_once("-")
                .and_then(|(start, end)| start.parse::<u128>().ok().zip(end.parse::<u128>().ok()))
        })
        .sorted_by_key(|&(start, _)| start)
        .coalesce(|(start_a, end_a), (start_b, end_b)| {
            if start_b <= end_a + 1 {
                Ok((start_a, end_a.max(end_b)))
            } else {
                Err(((start_a, end_a), (start_b, end_b)))
            }
        })
        .map(|(s, e)| e - s + 1)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() {
        //use std::fs;
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part1(&input_string);
        let result = resolve_part1(_EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn part2() {
        //use std::fs;
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part2(&input_string);
        let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 14);
    }
}
