use super::read_input::*;

pub fn runner() -> (usize, usize) {
    let input = read_input(6);
    let nums = input
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut days = [0usize; 9];
    nums.iter().for_each(|&x| days[x] += 1);

    let mut part1 = 0;

    for n in 0..256 {
        let v = days[0];
        for i in 1..9 {
            days[i - 1] = days[i];
        }
        days[6] += v;
        days[8] = v;
        if n == 79 {
            part1 = days.iter().sum();
        }
    }

    (part1, days.iter().sum())
}
