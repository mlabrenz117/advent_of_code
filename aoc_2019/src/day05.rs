use crate::intcode::*;

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<isize> {
    input.split(',').map(|s| s.parse::<isize>().unwrap()).collect()
}

#[aoc(day5, part1)]
fn part1(input: &[isize]) -> usize {
    let in_fn = Box::new(|| 1);
    let out_fn = Box::new(|x| {
        if x != 0 {
            println!("{}", x);
        }
    });
    let mut comp = IntcodeComputer::new(input, in_fn, out_fn);
    comp.run();
    0
}

#[aoc(day5, part2)]
fn part2(input: &[isize]) -> usize {
    let in_fn = Box::new(|| 5);
    let out_fn = Box::new(|x| println!("{}", x));
    let mut comp = IntcodeComputer::new(input, in_fn, out_fn);
    comp.run();
    0
}