use super::read_input::*;
use hashbrown::HashSet;
use itertools::Itertools;
use std::cmp::Ordering;

type Point = (isize, isize);

fn encode(p: Point) -> isize {
    if p.0 >= p.1 {
        p.0 * p.0 + p.0 + p.1
    } else {
        p.0 + p.1 * p.1
    }
}

pub fn runner() -> (usize, usize) {
    let input = read_input(5);
    let mut part1: HashSet<isize> = HashSet::new();
    let mut part2: HashSet<isize> = HashSet::new();
    let mut p1: HashSet<isize> = HashSet::new();
    let mut p2: HashSet<isize> = HashSet::new();
    let lines: Vec<(Point, Point)> = input
        .lines()
        .map(|x| {
            x.split("->")
                .map(|n| {
                    n.split(',')
                        .map(|v| v.trim().parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    for line in &lines {
        let mx = match line.0.0.cmp(&line.1.0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1
        };
        let my = match line.0.1.cmp(&line.1.1) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1
        };
        let mut p0 = line.0;
        let l = encode(p0);
        if p2.get(&l) == None {
            p2.insert(l);
        } else {
            part2.insert(l);
        }
        if mx == 0 || my == 0 {
            if p1.get(&l) == None {
                p1.insert(l);
            } else {
                part1.insert(l);
            }
        }
        while p0 != line.1 {
            p0.0 += mx;
            p0.1 += my;
            let l = encode(p0);
            if p2.get(&l) == None {
                p2.insert(l);
            } else {
                part2.insert(l);
            }
            if mx == 0 || my == 0 {
                if p1.get(&l) == None {
                    p1.insert(l);
                } else {
                    part1.insert(l);
                }
            }
        }
    }
    (
        part1.len(),
        part2.len(),
    )
}
