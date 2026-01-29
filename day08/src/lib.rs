use std::collections::HashMap;

const _EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(coordinates: &str) -> Self {
        let mut nums = coordinates
            .split(',')
            .map(|s: &str| s.trim().parse::<f64>().expect("Is not a number"));
        Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            z: nums.next().unwrap(),
        }
    }
}

struct Connection {
    d: f64,
    p1: usize,
    p2: usize,
}

impl Connection {
    pub fn new(a: Point, b: Point, index_a: usize, index_b: usize) -> Self {
        let distance = calculate_euclidean_distance(a, b);

        Self {
            d: distance,
            p1: index_a,
            p2: index_b,
        }
    }
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];

        Self { parent, size }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        } else {
            self.parent[i] = self.find(self.parent[i]);
            self.parent[i]
        }
    }

    pub fn unite(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
        }
    }
}

fn calculate_euclidean_distance(a: Point, b: Point) -> f64 {
    (b.x - a.x).powi(2) + (b.y - a.y).powi(2) + (b.z - a.z).powi(2)
}

pub fn resolve_part1(input_str: &str) -> usize {
    let points: Vec<Point> = input_str
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|line| Point::new(line))
        .collect();
    let plen = points.len();
    let num_connections = if plen == 20 { 10 } else { 1000 };

    let mut connections: Vec<Connection> = Vec::with_capacity(plen * (plen - 1) / 2);

    for i in 0..(plen - 1) {
        let a = points[i];
        for j in (i + 1)..plen {
            let b = points[j];
            let conn = Connection::new(a, b, i, j);
            connections.push(conn);
        }
    }

    connections.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

    let mut dsu = DSU::new(plen);
    for conn in connections.iter().take(num_connections) {
        dsu.unite(conn.p1, conn.p2);
    }

    let mut counts = HashMap::new();
    for i in 0..plen {
        let root = dsu.find(i);
        *counts.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = counts.values().cloned().collect();

    sizes.sort_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product()
}

pub fn resolve_part2(input_str: &str) -> u64 {
    let points: Vec<Point> = input_str
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|line| Point::new(line))
        .collect();
    let plen = points.len();
    let mut num_groups = plen;
    let mut last_pair_x_product = 0;

    let mut connections: Vec<Connection> = Vec::with_capacity(plen * (plen - 1) / 2);

    for i in 0..(plen - 1) {
        let a = points[i];
        for j in (i + 1)..plen {
            let b = points[j];
            let conn = Connection::new(a, b, i, j);
            connections.push(conn);
        }
    }

    connections.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());

    let mut dsu = DSU::new(plen);
    for conn in connections.iter() {
        let root_i = dsu.find(conn.p1);
        let root_j = dsu.find(conn.p2);

        if root_i != root_j {
            dsu.unite(conn.p1, conn.p2);
            num_groups -= 1;
        }

        if num_groups == 1 {
            let p1_x = points[conn.p1].x as u64;
            let p2_x = points[conn.p2].x as u64;
            last_pair_x_product = p1_x * p2_x;
            break;
        }
    }

    last_pair_x_product
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
        assert_eq!(result, 40);
    }

    #[test]
    fn part2() {
        //use std::fs;
        //let input_string = fs::read_to_string("input.txt").unwrap();
        //let result = resolve_part2(&input_string);
        let result = resolve_part2(_EXAMPLE);
        assert_eq!(result, 25272);
    }
}
