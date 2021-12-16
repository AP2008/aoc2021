use super::read_input::*;
use itertools::Itertools;

pub fn runner() -> (isize, isize) {
    let input = read_input(2);
    let num_arr = input
        .lines()
        .map(|line| {
            let k = line.split(' ').collect_vec();
            (k[0].to_string(), k[1].parse().unwrap())
        })
        .collect::<Vec<_>>();
    (part1(&num_arr), part2(&num_arr))
}

fn part1(num_arr: &[(String, isize)]) -> isize {
    let mut h = 0;
    let mut v = 0;
    num_arr.iter().for_each(|(s, i)| match s.as_str() {
        "down" => v += i,
        "up" => v -= i,
        "forward" => h += i,
        "backward" => h -= i,
        _ => (),
    });
    h * v
}

fn part2(num_arr: &[(String, isize)]) -> isize {
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;
    num_arr.iter().for_each(|(s, i)| match s.as_str() {
        "down" => aim += i,
        "up" => aim -= i,
        "forward" => {
            h += i;
            v += aim * i;
        }
        _ => (),
    });
    h * v
}
