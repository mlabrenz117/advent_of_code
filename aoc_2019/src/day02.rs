use crate::intcode::IntcodeComputer;

#[aoc_generator(day2)]
fn generator(input: &str) -> Vec<isize> {
    let mut input: Vec<isize> = input
        .split(',')
        .map(|v| v.parse::<isize>().unwrap())
        .collect();
    input[1] = 12;
    input[2] = 2;
    input
}

#[aoc(day2, part1)]
fn part1(input: &[isize]) -> isize {
    let in_iter = [];
    let out_fn = |_| {};
    let mut comp = IntcodeComputer::new(input, &in_iter, out_fn);
    comp.run().unwrap();
    let m = comp.memory();
    m[0]
}

#[aoc(day2, part2)]
fn part2(input: &[isize]) -> isize {
    let mut input = Vec::from(input);
    for noun in 0..=99 {
        for verb in 0..=99 {
            input[1] = noun;
            input[2] = verb;
            if part1(&input) == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn d02_p1() {
        let in1 = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(part1(&in1), 30);
    }
}
