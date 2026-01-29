const _EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

pub fn resolve_part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(u64::from))
                .collect();

            digits
                .iter()
                .enumerate()
                .flat_map(|(i, &d1)| digits[i + 1..].iter().map(move |&d2| d1 * 10 + d2))
                .max()
                .unwrap_or(0)
        })
        .sum()
}

pub fn resolve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u64> = line
                .chars()
                .filter_map(|c| c.to_digit(10).map(u64::from))
                .collect();

            let mut to_remove = digits.len() - 12;

            let mut stack = Vec::with_capacity(digits.len());

            for &digit in &digits {
                while let Some(&top) = stack.last() {
                    if top < digit && to_remove > 0 {
                        stack.pop();
                        to_remove -= 1;
                    } else {
                        break;
                    }
                }
                stack.push(digit);
            }

            stack.truncate(12);

            stack.into_iter().fold(0_u64, |acc, d| acc * 10 + d)
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1() {
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part1(&input_string);
        let result = resolve_part1(_EXAMPLE);
        assert_eq!(result, 357);
    }

    #[test]
    fn part2() {
        use std::fs;
        let input_string = fs::read_to_string("input.txt").unwrap();
        let result = resolve_part2(&input_string);
        //let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 3121910778619);
    }
}
