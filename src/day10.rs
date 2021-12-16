use crate::read_input::read_input;
use phf::phf_map;
use std::collections::VecDeque;

static CLOSES: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>'
};

static INCOMP_SCORES: phf::Map<char, usize> = phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4
};

static CORRUPT_SCORES: phf::Map<char, usize> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137
};

pub fn runner() -> (usize, usize) {
    let input = read_input(10);
    let mut corrupt = 0;
    let mut incomp = Vec::new();
    for line in input.lines() {
        let s = check(line.trim().to_string());
        if s.1 {
            corrupt += s.0;
        } else {
            incomp.push(s.0);
        }
    }
    incomp.sort_unstable();
    (corrupt, incomp[incomp.len() / 2])
}

fn check(s: String) -> (usize, bool) {
    let mut closed = VecDeque::new();
    for c in s.chars() {
        if let Some(&n) = CLOSES.get(&c) {
            closed.push_front(n);
        } else if Some(&c) == closed.front() {
            closed.pop_front();
        } else {
            return (CORRUPT_SCORES[&c], true);
        }
    }
    let mut score = 0;
    for i in closed {
        score *= 5;
        score += INCOMP_SCORES[&i];
    }
    (score, false)
}
