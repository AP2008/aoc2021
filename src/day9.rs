use crate::read_input::read_input;
use cached::proc_macro::cached;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref GRID: Vec<Vec<usize>> = get_grid();
}

type Point = (usize, usize);

fn encode(p: Point) -> usize {
    if p.0 >= p.1 {
        p.0 * p.0 + p.0 + p.1
    } else {
        p.0 + p.1 * p.1
    }
}

fn get_grid() -> Vec<Vec<usize>> {
    let input = read_input(9);
    let mut grid = Vec::new();
    for (i, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for n in line.chars() {
            grid[i].push(n.to_string().parse().unwrap());
        }
    }
    grid
}

pub fn runner() -> (usize, usize) {
    let mut res = Vec::new();
    let mut ne = Vec::new();
    for (i, row) in GRID.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            let mut t = true;
            if j != 0 && row[j - 1] <= col {
                t = false;
            }
            if j != row.len() - 1 && row[j + 1] <= col {
                t = false;
            }
            if i != 0 && GRID[i - 1][j] <= col {
                t = false;
            }
            if i != GRID.len() - 1 && GRID[i + 1][j] <= col {
                t = false;
            }
            if t {
                res.push(col);
                ne.push((i, j));
            }
        }
    }
    let mut ke = Vec::new();
    for p in ne {
        ke.push(recur(p).len());
    }
    ke.sort_unstable();
    (
        res.iter().sum::<usize>() + res.len(),
        ke[ke.len() - 3..ke.len()].iter().product::<usize>(),
    )
}

#[cached]
fn recur(loc: Point) -> HashSet<usize> {
    let mut h = HashSet::new();
    let v = GRID[loc.0][loc.1];
    if v == 9 {
        return h;
    }
    h.insert(encode(loc));
    if loc.0 != 0 && GRID[loc.0 - 1][loc.1] > v {
        h.extend(recur((loc.0 - 1, loc.1)).into_iter());
    }
    if loc.0 != GRID.len() - 1 && GRID[loc.0 + 1][loc.1] > v {
        h.extend(recur((loc.0 + 1, loc.1)).into_iter());
    }
    if loc.1 != 0 && GRID[loc.0][loc.1 - 1] > v {
        h.extend(recur((loc.0, loc.1 - 1)).into_iter());
    }
    if loc.1 != GRID[0].len() - 1 && GRID[loc.0][loc.1 + 1] > v {
        h.extend(recur((loc.0, loc.1 + 1)).into_iter());
    }
    h
}
