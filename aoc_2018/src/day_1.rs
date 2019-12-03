use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .collect()
}

#[aoc(day1, part1)]
pub fn one(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn two(input: &[i64]) -> i64 {
    let mut frequencies = HashSet::with_capacity(100000);
    let mut answer = 0;
    input.iter().cycle().try_fold(0, |acc, x| {
        let frequency = acc + x;
        if !frequencies.insert(frequency) {
            answer = frequency;
            None
        } else {
            Some(frequency)
        }
    });
    answer
}
