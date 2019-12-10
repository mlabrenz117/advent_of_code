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
    comp.run().unwrap();
    result as usize
}

#[aoc(day5, part2)]
fn part2(input: &[isize]) -> usize {
    let mut result = 0;
    let in_fn = [5];
    let out_fn = |x| result = x;
    let mut comp = IntcodeComputer::new(input, &in_fn[..], out_fn);
    comp.run().unwrap();
    result as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d5() {
        let mut result = -1;
        let programs = [
            "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9",
            "3,3,1105,-1,9,1101,0,0,12,4,12,99,1"];
        for program in &programs {
            let program = generator(program);
            let mut comp = IntcodeComputer::new(&program, &[5], |x| result = x);
            comp.run().unwrap();
            assert_eq!(result, 1);
            let mut comp = IntcodeComputer::new(&program, &[0], |x| result = x);
            comp.run().unwrap();
            assert_eq!(result, 0);
        }
    }
}
