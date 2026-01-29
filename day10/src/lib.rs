const _EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

pub fn resolve_part1(input_str: &str) -> i64 {
    input_str
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line
                .split(['[', ']', '(', ')', '{', '}'])
                .filter(|s| !s.trim().is_empty())
                .collect();

            let target_str = parts[0].trim();
            let n_lights = target_str.len();

            let target: Vec<u8> = target_str
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect();

            let mut buttons = Vec::new();
            for part in parts.iter().skip(1) {
                let trimmed_part = part.trim();
                if trimmed_part.starts_with(|c: char| char::is_digit(c, 10)) {
                    let mut btn = vec![0u8; n_lights];
                    for val_str in trimmed_part.split(',') {
                        match val_str.trim().parse::<usize>() {
                            Ok(idx) => {
                                if idx < n_lights {
                                    btn[idx] = 1;
                                }
                            }
                            Err(_) => println!("{val_str}"),
                        }
                    }
                    buttons.push(btn);
                }
            }

            let n_buttons = buttons.len();
            println!("{:?}", n_buttons);
            let mut min_presses = i64::MAX;

            for i in 0..(1 << n_buttons) {
                let mut current_state = vec![0_u8; n_lights];
                let mut count = 0;
                for b in 0..n_buttons {
                    if (i >> b) & 1 == 1 {
                        count += 1;
                        for l in 0..n_lights {
                            current_state[l] ^= buttons[b][l];
                        }
                    }
                }
                if current_state == target {
                    min_presses = min_presses.min(count);
                }
            }

            if min_presses == i64::MAX {
                0
            } else {
                min_presses
            }
        })
        .sum()
}

pub fn resolve_part2(_input_str: &str) -> i64 {
    0
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
        assert_eq!(result, 7);
    }

    // #[test]
    // fn part2() {
    //     //use std::fs;
    //     //let input_string = fs::read_to_string("input.txt").unwrap();
    //     //let result = resolve_part2(&input_string);
    //     let result = resolve_part2(_EXAMPLE);
    //     assert_eq!(result, 24);
    // }
}
