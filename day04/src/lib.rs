const _EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

pub fn resolve_part1(input_str: &str) -> usize {
    let input = input_str
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let len = input.len();
    (0..len)
        .flat_map(|row| (0..input[row].len()).map(move |col| (row, col)))
        .filter(|&(row, col)| {
            const DIRECTIONS: &[(isize, isize)] = &[
                (-1, 1),
                (0, 1),
                (1, 1),
                (-1, 0),
                (1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];

            if input[row][col] != b'@' {
                return false;
            }

            let sum: usize = DIRECTIONS
                .iter()
                .filter_map(|&(x, y)| {
                    let r = row.checked_add_signed(y)?;
                    let c = col.checked_add_signed(x)?;
                    if r >= input.len() || c >= input[r].len() {
                        return None;
                    }

                    if input[r][c] == b'@' { Some(1) } else { None }
                })
                .sum();

            sum < 4
        })
        .count()
}

pub fn resolve_part2(input_str: &str) -> usize {
    let mut grid = input_str
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let mut removed_count = 0;
    const DIRECTIONS: &[(isize, isize)] = &[
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    loop {
        let dying_cells: Vec<(usize, usize)> = (0..height)
            .flat_map(|row| (0..width).map(move |col| (row, col)))
            .filter(|&(row, col)| {
                if grid[row][col] != b'@' {
                    return false;
                }

                let neighbors_count: usize = DIRECTIONS
                    .iter()
                    .filter_map(|&(x, y)| {
                        let r = row.checked_add_signed(y)?;
                        let c = col.checked_add_signed(x)?;
                        match grid.get(r).and_then(|row| row.get(c)) {
                            Some(&b'@') => Some(1),
                            _ => None,
                        }
                    })
                    .sum();

                neighbors_count < 4
            })
            .collect();
        if dying_cells.is_empty() {
            break;
        }

        removed_count += dying_cells.len();

        for (r, c) in dying_cells {
            grid[r][c] = b'.';
        }
    }
    removed_count
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
        assert_eq!(result, 13);
    }

    #[test]
    fn part2() {
        //use std::fs;
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part2(&input_string);
        let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 43);
    }
}
