const _EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
L1000";

pub fn crosses_zero(start: i32, end: i32, delta: i32) -> bool {
    if end == 0 {
        return true;
    };

    if start == 0 {
        return false;
    };

    (delta > 0 && end < start) || (delta < 0 && end > start)
}

pub fn resolve_part2(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let direction = line.chars().next()?;
            let rotations = line[1..].parse::<i32>().ok()?;
            Some((direction, rotations))
        })
        .fold((50, 0), |(position, count), (direction, rotations)| {
            let delta = if direction == 'R' {
                rotations
            } else {
                -rotations
            };
            let new_position = (position + delta).rem_euclid(100);
            let new_count = delta.abs() / 100
                + if crosses_zero(position, new_position, delta) {
                    1
                } else {
                    0
                };
            (new_position, count + new_count)
        })
        .1
}

pub fn resolve_part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let direction = line.chars().next()?;
            let rotations = line[1..].parse::<i32>().ok()?;
            Some((direction, rotations))
        })
        .fold((50, 0), |(position, count), (direction, rotations)| {
            let new_position = match direction {
                'R' => (position + rotations).rem_euclid(100),
                'L' => (position - rotations).rem_euclid(100),
                _ => position,
            };
            let new_count = if new_position == 0 { count + 1 } else { count };
            (new_position, new_count)
        })
        .1
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn part1() {
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve(&input_string);
        let result = resolve_part1(_EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn part2() {
        let input_string = fs::read_to_string("input.txt").unwrap();
        let result = resolve_part2(&input_string);
        assert_eq!(result, 6166);
    }
}
