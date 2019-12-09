use crate::intcode::*;

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[isize]) -> usize {
    let mut result = 0;
    let in_fn = [1];
    let out_fn = |x| {
        if x != 0 {
            result = x;
        }
    };
    let mut comp = IntcodeComputer::new(input, &in_fn, out_fn);
    comp.run();
    result as usize
}

#[aoc(day5, part2)]
fn part2(input: &[isize]) -> usize {
    let mut result = 0;
    let in_fn = [5];
    let out_fn = |x| result = x;
    let mut comp = IntcodeComputer::new(input, &in_fn[..], out_fn);
    comp.run();
    result as usize
}
