use std::{borrow::Borrow, collections::HashMap};

#[derive(Debug)]
struct InitialState {
    pots: Pots,
    rules: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
struct Pots {
    pots_intern: String,
    zero: usize,
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Box<InitialState> {
    let mut lines = input.lines();
    let state = lines.next().unwrap()[15..].to_string();
    let state = Pots {
        pots_intern: state,
        zero: 0,
    };
    lines.next();
    let mut map = HashMap::new();
    for line in lines {
        let mut parts = line.split(" => ");
        let k = parts.next().unwrap().to_string();
        let v = parts.next().unwrap();
        let v = {
            if v == "#" {
                true
            } else {
                false
            }
        };
        map.insert(k, v);
    }
    Box::new(InitialState {
        pots: state,
        rules: map,
    })
}

#[aoc(day12, part1)]
fn solve(init: &InitialState) -> isize {
    part1(init, 20)
}

#[aoc(day12, part2)]
fn solve2(init: &InitialState) -> usize {
    dbg!(part1(init, 50));
    dbg!(part1(init, 500));
    dbg!(part1(init, 5000));
    0
}

fn part1(init: &InitialState, gen: usize) -> isize {
    let mut last_gen = Box::new(init.pots.clone());
    let mut next_gen = Box::new(Pots::new());
    for _ in 0..gen {
        let mut minus_two = String::from("....");
        minus_two.push(last_gen.pots_intern.chars().nth(0).unwrap_or('.'));
        let minus_two = init.rules.get(&minus_two).unwrap_or(&false);
        let mut minus_one = String::from("...");
        minus_one.push_str(&last_gen.pots_intern[0..2]);
        let minus_one = init.rules.get(&minus_one).unwrap_or(&false);
        let mut zero = String::from("..");
        zero.push_str(&last_gen.pots_intern[0..3]);
        let zero = init.rules.get(&zero).map(bool_to_char).unwrap_or('.');
        let mut one = String::from(".");
        one.push_str(&last_gen.pots_intern[0..4]);
        let one = init.rules.get(&one).map(bool_to_char).unwrap_or('.');
        if *minus_two {
            next_gen.pots_intern.push('#');
            next_gen.zero += 2;
        }
        if *minus_one {
            next_gen.pots_intern.push('#');
            next_gen.zero += 1;
        }
        if *minus_two && *minus_one {
            next_gen.zero -= 1;
        }
        next_gen.pots_intern.push(zero);
        next_gen.pots_intern.push(one);

        for i in 2..=last_gen.pots_intern.len() - 3 {
            match init
                .rules
                .get(&last_gen.pots_intern[i - 2..=i + 2])
                .map(bool_to_char)
            {
                Some(v) => {
                    next_gen.pots_intern.push(v);
                }
                None => next_gen.pots_intern.push('.'),
            }
        }
        // last 4;
        let mut v = Vec::new();
        let n = last_gen.pots_intern.len();
        let mut n_minus_2 = String::from(&last_gen.pots_intern[n - 4..]);
        n_minus_2.push('.');
        v.push(n_minus_2);
        let mut n_minus_1 = String::from(&last_gen.pots_intern[n - 3..]);
        n_minus_1.push_str("..");
        v.push(n_minus_1);
        let mut n0 = String::from(&last_gen.pots_intern[n - 2..]);
        n0.push_str("...");
        v.push(n0);
        let mut n_plus_one = String::from(&last_gen.pots_intern[n - 1..]);
        n_plus_one.push_str("....");
        v.push(n_plus_one);

        v.iter()
            .map(|v| init.rules.get(v).unwrap_or(&false))
            .map(bool_to_char)
            .for_each(|v| next_gen.pots_intern.push(v));

        loop {
            match next_gen.pots_intern.chars().last() {
                Some(c) if c == '.' => {
                    next_gen.pots_intern.pop();
                }
                _ => break,
            }
        }

        last_gen = next_gen;
        next_gen = Box::new(Pots::new());
        next_gen.zero = last_gen.zero;
    }
    last_gen.zero -= 1;
    last_gen.num_plants()
}

fn bool_to_char<B>(b: B) -> char
where
    B: Borrow<bool>,
{
    if *b.borrow() {
        '#'
    } else {
        '.'
    }
}

impl Pots {
    fn new() -> Self {
        Pots {
            pots_intern: String::new(),
            zero: 0,
        }
    }

    fn num_plants(&self) -> isize {
        let mut sum = 0;
        for (i, c) in self.pots_intern.chars().enumerate() {
            let v: isize = i as isize - self.zero as isize - 1isize;
            if c == '#' {
                sum += v;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn t() {
        let pot = Pots {
            pots_intern: String::from(".#....##....#####...#######....#.#..##."),
            zero: 2,
        };
        assert_eq!(pot.num_plants(), 325);
    }

    #[test]
    fn t1() {
        let mut f = File::open("./input/2018/day12_test.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let input = generator(&s);
        assert_eq!(part1(&input, 20), 325);
    }
}
