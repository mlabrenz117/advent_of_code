use std::collections::HashMap;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[aoc(day2, part1)]
pub fn one(input: &[String]) -> u64 {
    let (twos, threes) = input.iter().fold((0, 0), |acc, x| {
        let (two, three) = count(x);
        (acc.0 + two as u64, acc.1 + three as u64)
    });
    twos * threes
}

#[aoc(day2, part2)]
pub fn two(input: &[String]) -> String {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if let Some(c) = off_by_one(&input[i], &input[j]) {
                let mut v = input[i].clone();
                v.remove(c);
                return v;
            }
        }
    }
    "Hello".to_owned()
}

fn count(id: &str) -> (bool, bool) {
    let mut map = HashMap::new();
    id.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    map.values().fold((false, false), |acc, &x| {
        ((x == 2 || acc.0), (x == 3 || acc.1))
    })
}

fn off_by_one(a: &str, b: &str) -> Option<usize> {
    let mut bad = false;
    let mut diff_by_one = false;
    let mut candidate = 0;
    let chars = a.chars().zip(b.chars()).enumerate();
    chars.for_each(|(i, (a, b))| {
        if a != b {
            if !diff_by_one {
                diff_by_one = true;
                candidate = i;
            } else {
                bad = true;
            }
        }
    });
    if diff_by_one && !bad {
        Some(candidate)
    } else {
        None
    }
}
