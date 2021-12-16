use super::read_input::*;

pub fn runner() -> (usize, usize) {
    let input = read_input(1);
    let nums = input
        .lines()
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let count1 = nums.windows(2).filter(|x| x[0] < x[1]).count();
    let count2 = nums.windows(4).filter(|x| x[0] < x[3]).count();
    (count1, count2)
}
