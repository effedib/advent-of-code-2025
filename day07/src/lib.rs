use std::collections::HashSet;

const _EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

pub fn resolve_part1(input_str: &str) -> u64 {
    let grid: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len();

    let mut counter = 1_u64;

    let first_splitter = grid[0]
        .iter()
        .position(|&s| s == 'S')
        .expect("No original split found");

    let mut current_beams = HashSet::new();

    let split_tachyon = |beams: &mut HashSet<usize>, index: usize| {
        if index > 0 {
            beams.insert(index - 1);
        }
        if index < (width - 1) {
            beams.insert(index + 1);
        }
    };

    split_tachyon(&mut current_beams, first_splitter);

    for row in grid.iter().skip(1) {
        let mut next_beams = HashSet::new();
        let mut already_counted = HashSet::new();
        for &col_index in &current_beams {
            if row[col_index] == '^' {
                if already_counted.insert(col_index) {
                    counter += 1;
                }
                split_tachyon(&mut next_beams, col_index);
            } else {
                next_beams.insert(col_index);
            }
        }
        current_beams = next_beams;
    }
    counter
}

pub fn resolve_part2(input_str: &str) -> u128 {
    let mut lines = input_str.lines();
    let first_line = lines.next().expect("Invalid input");
    let width = first_line.len();

    let mut initial_counts = vec![0_u128; width];

    if let Some(s_idx) = first_line.find('S') {
        if s_idx > 0 {
            initial_counts[s_idx - 1] = 1;
        }
        if s_idx < width - 1 {
            initial_counts[s_idx + 1] = 1;
        }
    }

    let final_counts = lines.fold(initial_counts, |current, row_str| {
        let mut next = vec![0_u128; width];

        for (i, (count, ch)) in current.into_iter().zip(row_str.chars()).enumerate() {
            if count == 0 {
                continue;
            }
            if ch == '^' {
                if i > 0 {
                    next[i - 1] += count;
                }
                if i < width - 1 {
                    next[i + 1] += count;
                }
            } else {
                next[i] += count;
            }
        }
        next
    });

    final_counts.into_iter().sum()
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
        assert_eq!(result, 21);
    }

    #[test]
    fn part2() {
        //use std::fs;
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part2(&input_string);
        let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 40);
    }
}
