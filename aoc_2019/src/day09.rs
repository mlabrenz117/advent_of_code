use crate::intcode::*;

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<isize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(program: &[isize]) -> isize {
    let mut result = 0;
    let input = [1];
    let output_fn = |x| {
        result = x;
    };
    let mut comp = IntcodeComputer::new(program, &input, output_fn);
    comp.run().unwrap();
    result
}

#[aoc(day9, part2)]
fn part2(program: &[isize]) -> isize {
    let mut result = 0;
    let input = [2];
    let output_fn = |x| {
        result = x;
    };
    let mut comp = IntcodeComputer::new(program, &input, output_fn);
    comp.run().unwrap();
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d9p1() {
        let mut out = 0;
        let input1 = "104,1125899906842624,99";
        let input = [1];
        let output_fn = |x| out = x;
        let program = generator(input1);
        let mut comp = IntcodeComputer::new(&program, &input, output_fn);
        comp.run().unwrap();
        assert_eq!(out, 1125899906842624);
        let output_fn = |x| out = x;
        let input2 = "1102,34915192,34915192,7,4,7,99,0";
        let program = generator(input2);
        let mut comp = IntcodeComputer::new(&program, &input, output_fn);
        comp.run().unwrap();
        assert_eq!(16, out.to_string().len());
        let input3 = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut out_vec: Vec<isize> = Vec::with_capacity(16);
        let out_fn = |x| out_vec.push(x);
        let program = generator(input3);
        let mut comp = IntcodeComputer::new(&program, &input, out_fn);
        comp.run().unwrap();
        assert_eq!(out_vec, program);
    }
}
