use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    str::FromStr,
};

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|l| {
            let mut words = l.split_whitespace();
            (
                char::from_str(words.nth(1).unwrap()).unwrap(),
                char::from_str(words.nth(5).unwrap()).unwrap(),
            )
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn order(requirements: &[(char, char)]) -> String {
    let mut result = String::with_capacity(32);
    let mut map = HashMap::with_capacity(32);
    let mut set = HashSet::with_capacity(32);
    for (r0, r1) in requirements {
        set.insert(r0);
        set.insert(r1);
        map.entry(r1).or_insert(Vec::new()).push(*r0);
    }
    while !set.is_empty() {
        for c in b'A'..=b'Z' {
            let c = char::from(c);
            if set.contains(&c)
                && map
                    .get(&c)
                    .map_or(true, |req| req.iter().all(|x| !set.contains(x)))
            {
                result.push(c);
                set.remove(&c);
                break;
            }
        }
    }
    result
}

#[aoc(day7, part2)]
pub fn part2(requirements: &[(char, char)]) -> usize {
    solve(requirements, 5)
}

fn solve(requirements: &[(char, char)], workers: usize) -> usize {
    let mut map = HashMap::with_capacity(32);
    let mut set = HashSet::with_capacity(32);
    for (r0, r1) in requirements {
        set.insert(Step::new(*r0));
        set.insert(Step::new(*r1));
        map.entry(r1).or_insert(Vec::new()).push(*r0);
    }

    let mut i = 0;
    let mut workers: Vec<Option<*mut Step>> = vec![None; workers];
    while !set.is_empty() {
        for worker in workers.iter_mut() {
            if let Some(step) = worker {
                let step: &mut Step = unsafe { &mut **step };
                if step.done() {
                    *worker = None;
                    set.remove(step);
                } else {
                    step.decrement();
                }
            }
            if worker.is_none() {
                *worker = {
                    let mut r = None;
                    for c in b'A'..=b'Z' {
                        let c = char::from(c);
                        if set.contains(&c)
                            && !set.get(&c).unwrap().in_progress()
                            && map
                                .get(&c)
                                .map_or(true, |req| req.iter().all(|x| !set.contains(x)))
                        {
                            let s = set.get(&c).unwrap();
                            let s: *mut Step = (s as *const Step) as *mut Step;
                            unsafe {
                                (*s).start();
                            }
                            r = Some(s);
                            break;
                        }
                    }
                    r
                };
            }
        }
        i += 1;
    }
    i - 1
}

struct Step {
    name: char,
    in_progress: bool,
    time: u8,
}

impl Step {
    fn new(name: char) -> Self {
        Self {
            name,
            in_progress: false,
            time: time(name) - 1,
        }
    }

    fn decrement(&mut self) {
        self.time -= 1;
    }

    fn in_progress(&self) -> bool {
        self.in_progress
    }

    fn start(&mut self) {
        self.in_progress = true;
    }

    fn done(&self) -> bool {
        self.time == 0
    }
}

impl Hash for Step {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Borrow<char> for Step {
    fn borrow(&self) -> &char {
        &self.name
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Step {}

fn time(c: char) -> u8 {
    let mut b = [0; 1];
    c.encode_utf8(&mut b);
    b[0] - 4
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let v = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ];

        let s = order(&v);
        assert_eq!(s, "CABDFE");
    }

    #[test]
    fn test_time() {
        assert_eq!(61, time('A'));
        assert_eq!(86, time('Z'));
    }

    #[test]
    fn part2() {
        let v = vec![
            ('C', 'A'),
            ('C', 'F'),
            ('A', 'B'),
            ('A', 'D'),
            ('B', 'E'),
            ('D', 'E'),
            ('F', 'E'),
        ];

        let s = solve(&v, 2);
        assert_eq!(s, 258);
    }
}
