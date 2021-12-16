#![feature(iter_advance_by)]

use aoc2021::{day1, day10, day2, day3, day5, day6, day7, day8, day9};
fn main() {
    println!("Advent Of Code 2021\n\n");
    let d1 = day1::runner();
    let d2 = day2::runner();
    let d3 = day3::runner();
    let d5 = day5::runner();
    let d6 = day6::runner();
    let d7 = day7::runner();
    let d8 = day8::runner();
    let d9 = day9::runner();
    let d10 = day10::runner();
    println!("Day 1\nPart 1: {}\nPart 2: {}\n", d1.0, d1.1);
    println!("Day 2\nPart 1: {}\nPart 2: {}\n", d2.0, d2.1);
    println!("Day 3\nPart 1: {}\nPart 2: {}\n", d3.0, d3.1);
    println!("Day 5\nPart 1: {}\nPart 2: {}\n", d5.0, d5.1);
    println!("Day 6\nPart 1: {}\nPart 2: {}\n", d6.0, d6.1);
    println!("Day 7\nPart 1: {}\nPart 2: {}\n", d7.0, d7.1);
    println!("Day 8\nPart 1: {}\nPart 2: {}\n", d8.0, d8.1);
    println!("Day 9\nPart 1: {}\nPart 2: {}\n", d9.0, d9.1);
    println!("Day 10\nPart 1: {}\nPart 2: {}\n", d10.0, d10.1);
}
