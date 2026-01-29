const _EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

pub fn resolve_part1(input_str: &str) -> u128 {
    let grid: Vec<Vec<&str>> = input_str
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let row_len = grid.len();
    let col_len = grid[0].len();
    let operator_row = row_len - 1;

    (0..col_len)
        .map(|col| {
            let operator = grid[operator_row][col];

            let mut numbers =
                (0..row_len - 1).map(|row| grid[row][col].parse::<u128>().expect("Not a number"));

            let first_number = numbers.next().unwrap_or(0);

            numbers.fold(first_number, |acc, num| match operator {
                "+" => acc + num,
                "*" => acc * num,
                _ => panic!("Only + & * excpected, but found: {}", operator),
            })
        })
        .sum()
}

pub fn resolve_part2(input_str: &str) -> u64 {
    let mut lines: Vec<Vec<char>> = input_str.lines().map(|l| l.chars().collect()).collect();

    // In Rust dobbiamo gestire manualmente il fatto che le righe
    // potrebbero non avere la stessa lunghezza (Python lo gestisce o va in errore).
    // Qui prendiamo gli operatori dall'ultima riga.
    let operators = lines.pop().expect("Input vuoto");
    let max_len = operators.len();

    let mut ans: u64 = 0;
    let mut values: Vec<u64> = Vec::new();

    // Iteriamo le colonne al contrario come nel tuo Python
    for col in (0..max_len).rev() {
        let mut val_str = String::new();

        // Iteriamo le righe (che ora sono solo i numeri)
        for row in &lines {
            // Controllo bounds check manuale (in Python try/catch o implicito)
            if col < row.len() {
                let digit = row[col];
                if digit != ' ' {
                    val_str.push(digit);
                }
            }
        }

        // Se la colonna era vuota (separatore)
        if val_str.is_empty() {
            values.clear(); // Resetta per il prossimo problema
            continue;
        }

        // Parsing del numero
        if let Ok(num) = val_str.parse::<u64>() {
            values.push(num);
        }

        // Logica operatore
        let op = operators[col];
        if op == ' ' {
            continue;
        } else if op == '+' {
            ans += values.iter().sum::<u64>();
        } else if op == '*' {
            ans += values.iter().product::<u64>();
        }
    }

    ans
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
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part2() {
        //use std::fs;
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part2(&input_string);
        let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 3263827);
    }
}
