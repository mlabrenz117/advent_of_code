use crate::utils::HashMapExtension;
use std::{collections::HashMap, convert::TryInto};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[aoc_generator(day3)]
fn generator(input: &str) -> Vec<HashMap<Point, usize>> {
    let mut cables = Vec::new();
    input.lines().enumerate().for_each(|(cable, line)| {
        let mut cursor = Point { x: 0, y: 0 };
        let mut steps = 0;
        cables.push(HashMap::new());
        line.split(',')
            .map(|v| {
                (
                    Direction::from(v.chars().nth(0).unwrap()),
                    v[1..].parse::<usize>().unwrap(),
                )
            })
            .for_each(|(dir, value)| {
                for _ in 0..value {
                    cursor.step(&dir);
                    steps += 1;
                    let map = cables.get_mut(cable).unwrap();
                    if let Some(old) = map.insert(cursor, steps) {
                        map.insert(cursor, old);
                    }
                }
            });
    });
    cables
}

fn solve<F>(input: &[HashMap<Point, usize>], func: F) -> usize
where
    F: FnMut(&Point) -> usize,
{
    input[0].intersection(&input[1]).map(func).min().unwrap()
}

#[aoc(day3, part1)]
fn part1(input: &[HashMap<Point, usize>]) -> usize {
    solve(input, |p| p.manhatten())
}

#[aoc(day3, part2)]
fn part2(input: &[HashMap<Point, usize>]) -> usize {
    solve(input, |point| {
        input[0].get(point).unwrap() + input[1].get(point).unwrap()
    })
}

impl Point {
    fn step(&mut self, dir: &Direction) {
        match dir {
            Direction::East => self.x += 1,
            Direction::West => self.x -= 1,
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
        }
    }

    fn manhatten(&self) -> usize {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }
}

impl From<char> for Direction {
    fn from(input: char) -> Self {
        match input {
            'R' => Self::East,
            'D' => Self::South,
            'U' => Self::North,
            'L' => Self::West,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d3() {
        let mut input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let mut parsed = generator(input);
        assert_eq!(part1(&parsed[..]), 6);
        assert_eq!(part2(&parsed[..]), 30);
        input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        parsed = generator(input);
        assert_eq!(part1(&parsed[..]), 159);
        assert_eq!(part2(&parsed[..]), 610);
        input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        parsed = generator(input);
        assert_eq!(part1(&parsed[..]), 135);
        assert_eq!(part2(&parsed[..]), 410);
    }
}
