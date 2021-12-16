use regex;
use super::read_input::*;
use itertools::Itertools;

pub fn runner() -> (isize, isize) {
    let input = read_input(5);
    let lines = input.lines().collect_vec();

}

fn check_grid(grid: Vec<isize>) -> bool {
    let l = (grid.len() as f64).sqrt() as usize;
    for i in 0..l {
        if grid[l * i..l * (i + 1)].iter().sum::<isize>() == -(l as isize)
            || (0..l).map(|x| grid[l * x + i]).sum::<isize>() == -(l as isize)
        {
            return true;
        }
    }
    false
}
