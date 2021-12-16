use super::read_input::*;
use itertools::Itertools;

pub fn runner() -> (isize, isize) {
    let input = read_input(3);
    let mut x = vec![];
    for l in input.lines() {
        let c = l.chars();
        x.push(c.map(|n| n.to_string().parse::<usize>().unwrap()).collect());
    }
    let mut p = x.clone();
    let mut q = x.clone();
    p = bitter(&p).0;
    q = bitter(&q).1;
    let mut g = 0;
    let mut h = 0;
    for k in p[0].iter() {
        g <<= 1;
        g += k;
    }
    for k in q[0].iter() {
        h <<= 1;
        h += k;
    }
    ((g * h) as isize, 0)
}

fn bitter(arr: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut t = arr.clone();
    let mut i = 0;
    while t.len() != 1 {
        let mut x = vec![];
        for l in &t {
            for (i, n) in l.iter().enumerate() {
                if i >= x.len() {
                    x.push(vec![]);
                }
                x[i].push(n);
            }
        }
        let p = x
            .iter()
            .map(|n| {
                if n.iter().filter(|x| ***x == 0).count() > n.iter().filter(|x| ***x == 1).count() {
                    0
                } else {
                    1
                }
            })
            .collect_vec();
        if t.len() == 1 {
            break;
        }
        t = t
            .iter()
            .filter(|x| x[i] == p[i])
            .map(|n| n.clone())
            .collect::<Vec<_>>();
        i += 1;
    }
    let mut m = arr.clone();
    i = 0;
    while m.len() != 1 {
        let mut x = vec![];
        for l in &m {
            for (i, n) in l.iter().enumerate() {
                if i >= x.len() {
                    x.push(vec![]);
                }
                x[i].push(n);
            }
        }
        let q = x
            .iter()
            .map(|n| {
                if n.iter().filter(|x| ***x == 0).count() > n.iter().filter(|x| ***x == 1).count() {
                    1
                } else {
                    0
                }
            })
            .collect_vec();
        if m.len() == 1 {
            break;
        }
        m = m
            .iter()
            .filter(|x| x[i] == q[i])
            .map(|n| n.clone())
            .collect::<Vec<_>>();
        i += 1;
    }
    (t.to_vec(), m.to_vec())
}
