use crate::read_input::read_input;

pub fn runner() -> (isize, isize) {
    let input = read_input(7);
    let mut nums = input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<_>>();
    nums.sort_unstable();
    (part1(&nums), part2(&nums))
}

fn part1(nums: &[isize]) -> isize {
    let n = nums[(nums.len() + 1) / 2];
    let mut align = 0;
    for &m in nums {
        align += (m - n).abs();
    }
    align
}

fn part2(nums: &[isize]) -> isize {
    let mut min = isize::MAX;
    let s = nums.iter().sum::<isize>();
    let v = s / (nums.len() as isize);
    for n in v..=(v + 1) {
        let align = nums
            .iter()
            .map(|&m| {
                let dist = (m - n).abs();
                dist * (dist + 1) / 2
            })
            .sum::<isize>();
        if align < min {
            min = align;
        }
    }
    min
}
