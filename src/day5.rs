use super::read_input::*;
use std::cmp::Ordering;

type Point = (i32, i32);

fn sign(d: i32) -> i32 {
    match d.cmp(&0) {
        Ordering::Greater => -1,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

fn read_line(v: &mut std::slice::Iter<u8>) -> Option<(Point, Point)> {
    let x1 = read_num(v);
    if x1 == None { return None; }
    let x1 = x1.unwrap();
    let y1 = read_num(v).unwrap();
    v.advance_by(3).unwrap();
    let x2 = read_num(v).unwrap();
    let y2 = read_num(v).unwrap();
    Some(((x1, y1), (x2, y2)))
}

fn read_num(v: &mut std::slice::Iter<u8>) -> Option<i32> {
    let mut num = 0;
    let mut empty = true;
    for &x in v {
        empty = false;
        if (48..=57).contains(&x) {
            num = num * 10 + ((x - 48) as i32);
        } else {
            break;
        }
    }
    if empty {
        None
    } else {
        Some(num)
    }
}

pub fn runner() -> (u32, u32) {
    let bytes = read_input_bytes(5);
    let mut bi = bytes.iter();
    let mut matrix1 = [0u8; 1000000];
    let mut matrix2 = [0u8; 1000000];
    while let Some(line) = read_line(&mut bi) {
        let dx = sign(line.0 .0 - line.1 .0);
        let dy = sign(line.0 .1 - line.1 .1);
        let mut p0 = line.0;
        let p1 = (line.1 .0 + dx, line.1 .1 + dy);
        while p0 != p1 {
            if dx == 0 || dy == 0 {
                matrix1[(p0.1 * 1000 + p0.0) as usize] += 1;
            } else {
                matrix2[(p0.1 * 1000 + p0.0) as usize] += 1;
            }
            p0.0 += dx;
            p0.1 += dy;
        }
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..1000000 {
        if matrix1[i] > 1 {
            p1 += 1;
        } else if matrix1[i] + matrix2[i] > 1 {
            p2 += 1;
        }
    }
    (p1, p2 + p1)
}
