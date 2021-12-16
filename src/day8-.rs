use crate::read_input::read_input;
use std::collections::HashMap;
use phf::phf_map;

static DIGITS: phf::Map<&'static str, usize> = phf_map! {
    "abcefg" => 0,
    "cf" => 1,
    "acdeg" => 2,
    "acdfg" => 3,
    "bcdf" => 4,
    "abdfg" => 5,
    "abdefg" => 6,
    "acf" => 7,
    "abcdefg" => 8,
    "abcdfg" => 9
};

pub fn runner() -> (usize, usize) {
    let input = read_input(8);
    let mut count = 0;
    let mut s = 0;
    for line in input.lines() {
        let k = line.split(" | ").collect::<Vec<_>>()[0].split(' ').map(|n| n.trim().chars().collect()).collect();
        let n = line.split(" | ").collect::<Vec<_>>()[1].split(' ').map(|n| n.trim().chars().collect()).collect();
        s += part2(k, n);
        for s in line.split('|').collect::<Vec<_>>()[1].split(' ') {
            match s.trim().len() {
                3 | 2 | 7 | 4 => count += 1,
                _ => ()
            }
        }
    }
    (count, s)
}

fn part2(signal_ps: Vec<Vec<char>>, output_vals: Vec<Vec<char>>) -> usize {
    let mut seg_7 = HashMap::with_capacity(7);
    let mut sized = vec![Vec::new(); 8]; // 2, 3, 4, 5, 6, 7
    for signal in signal_ps {
        sized[signal.len()].push(signal);
    }
    let elem2 = sized[2].get(0).unwrap();
    let elem3 = sized[3].get(0).unwrap().iter().filter(|x| !elem2.contains(x)).collect::<Vec<_>>();
    let elem4 = sized[4].get(0).unwrap().iter().filter(|x| !elem2.contains(x)).collect::<Vec<_>>();
    let v3 = sized[5].iter().find(|n| n.contains(&elem2[0]) && n.contains(&elem2[1])).unwrap();
    let x1 = v3.iter().find(|x| elem4.contains(x)).unwrap();
    let x2 = elem4.iter().find(|x| !v3.contains(x)).unwrap();
    let v5 = sized[5].iter().find(|x| x.contains(x1) && x.contains(x2)).unwrap();
    seg_7.insert(x1, 'd');
    seg_7.insert(x2, 'b');
    seg_7.insert(elem3[0], 'a');
    if v5.contains(&elem2[0]) {
        seg_7.insert(&elem2[0], 'f');
        seg_7.insert(&elem2[1], 'c');
    } else {
        seg_7.insert(&elem2[0], 'c');
        seg_7.insert(&elem2[1], 'f');
    }
    seg_7.insert(v3.iter()
        .find(|x| !elem4.contains(x) && !sized[3][0].contains(x)).unwrap(), 'g');
    let mut s = 0;
    let r = 'e';
    for t in output_vals {
        let mut n = Vec::new();
        for k in t {
            n.push(seg_7.get(&k).unwrap_or(&r).to_string());
        }
        n.sort_unstable();
        s *= 10;
        s += DIGITS[&n.join("")];
    }
    s
}
