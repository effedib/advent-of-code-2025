const _EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(coordinates: &str) -> Self {
        let mut nums = coordinates
            .split(',')
            .map(|s: &str| s.trim().parse::<i64>().expect("Is not a number"));
        Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
        }
    }
}

fn calculate_rectangle_area(a: &Point, b: &Point) -> i64 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

pub fn resolve_part1(input_str: &str) -> i64 {
    let points: Vec<Point> = input_str
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|line| Point::new(line))
        .collect();

    points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            points[i + 1..]
                .iter()
                .map(move |b| calculate_rectangle_area(a, b))
        })
        .max()
        .unwrap_or(0)
}

pub fn resolve_part2(input_str: &str) -> i64 {
    let red_points: Vec<(i64, i64)> = input_str
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            Some((
                parts.next()?.trim().parse().ok()?,
                parts.next()?.trim().parse().ok()?,
            ))
        })
        .collect();

    let plen = red_points.len();
    if plen == 0 {
        return 0;
    }

    // 1. Discretizzazione con "spessore"
    // Per essere sicuri di non perdere la forma, usiamo le X e Y dei punti rossi
    let mut xs: Vec<i64> = red_points.iter().map(|p| p.0).collect();
    let mut ys: Vec<i64> = red_points.iter().map(|p| p.1).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();

    // 2. Muri verticali per il riempimento
    let mut vertical_segments = Vec::new();
    for i in 0..plen {
        let p1 = red_points[i];
        let p2 = red_points[(i + 1) % plen];
        if p1.0 == p2.0 {
            vertical_segments.push((p1.0, p1.1.min(p2.1), p1.1.max(p2.1)));
        }
    }

    // 3. Creazione della matrice binaria basata sulla griglia discretizzata
    let mut grid = vec![vec![false; xs.len()]; ys.len()];
    for (y_idx, &y) in ys.iter().enumerate() {
        let mut row_intersections: Vec<i64> = vertical_segments
            .iter()
            .filter(|&&(_, y_min, y_max)| y >= y_min && y < y_max)
            .map(|&s| s.0)
            .collect();
        row_intersections.sort_unstable();

        for chunk in row_intersections.chunks_exact(2) {
            let x_start = chunk[0];
            let x_end = chunk[1];
            for (x_idx, &x) in xs.iter().enumerate() {
                if x >= x_start && x <= x_end {
                    grid[y_idx][x_idx] = true;
                }
            }
        }
        // Includiamo i bordi orizzontali espliciti (fondamentale per le righe piene)
        for i in 0..plen {
            let p1 = red_points[i];
            let p2 = red_points[(i + 1) % plen];
            if p1.1 == p2.1 && p1.1 == y {
                let x_min = p1.0.min(p2.0);
                let x_max = p1.0.max(p2.0);
                for (x_idx, &x) in xs.iter().enumerate() {
                    if x >= x_min && x <= x_max {
                        grid[y_idx][x_idx] = true;
                    }
                }
            }
        }
    }

    // 4. Invece dell'istogramma (che soffre la discretizzazione non uniforme),
    // torniamo al controllo delle coppie di punti ma ottimizzato dalla SAT discretizzata.

    // Costruiamo la SAT sulla griglia discretizzata
    let mut sat = vec![vec![0i64; xs.len() + 1]; ys.len() + 1];
    for r in 0..ys.len() {
        for c in 0..xs.len() {
            let val = if grid[r][c] { 1 } else { 0 };
            sat[r + 1][c + 1] = val + sat[r][c + 1] + sat[r + 1][c] - sat[r][c];
        }
    }

    let mut max_area = 0;
    // Iteriamo solo sui punti rossi reali
    for i in 0..plen {
        for j in i + 1..plen {
            let p1 = red_points[i];
            let p2 = red_points[j];

            let x_min = p1.0.min(p2.0);
            let x_max = p1.0.max(p2.0);
            let y_min = p1.1.min(p2.1);
            let y_max = p1.1.max(p2.1);

            // Troviamo gli indici corrispondenti nella nostra griglia compressa
            let ix1 = xs.binary_search(&x_min).unwrap();
            let ix2 = xs.binary_search(&x_max).unwrap();
            let iy1 = ys.binary_search(&y_min).unwrap();
            let iy2 = ys.binary_search(&y_max).unwrap();

            // Numero di celle discretizzate che dovrebbero essere piene
            let expected_cells = ((ix2 - ix1 + 1) * (iy2 - iy1 + 1)) as i64;
            let actual_cells =
                sat[iy2 + 1][ix2 + 1] - sat[iy1][ix2 + 1] - sat[iy2 + 1][ix1] + sat[iy1][ix1];

            if actual_cells == expected_cells {
                let area = (x_max - x_min + 1) * (y_max - y_min + 1);
                max_area = max_area.max(area);
            }
        }
    }

    max_area
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
        assert_eq!(result, 50);
    }

    #[test]
    fn part2() {
        //use std::fs;
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part2(&input_string);
        let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 24);
    }
}
