use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rectangle> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    }

    input
        .lines()
        .map(|string| {
            let mut values = RE
                .find_iter(string)
                .map(|num| num.as_str().parse::<usize>().unwrap());
            let id = values.next().unwrap();
            let left = values.next().unwrap();
            let top = values.next().unwrap();
            let width = values.next().unwrap();
            let height = values.next().unwrap();
            Rectangle::new(id, left, top, width, height)
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn one(inputs: &[Rectangle]) -> usize {
    let mut points = HashMap::new();
    for rec in inputs {
        for point in rec.points() {
            *points.entry(point).or_insert(0) += 1;
        }
    }
    points.iter().filter(|(_, &v)| v >= 2).count()
}

#[aoc(day3, part2)]
pub fn two(inputs: &[Rectangle]) -> usize {
    let mut rectangles = inputs.to_vec();
    for i in 0..rectangles.len() {
        if !rectangles[i].overlaps {
            for j in i + 1..rectangles.len() {
                if rectangles[i].overlaps(&rectangles[j]) {
                    rectangles.get_mut(i).unwrap().overlaps = true;
                    rectangles.get_mut(j).unwrap().overlaps = true;
                }
            }
        }
    }
    rectangles
        .iter()
        .filter(|r| !r.overlaps)
        .map(|r| r.id)
        .next()
        .expect("Could not find non-overlapping rectangles.")
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    id: usize,
    top_l: Point,
    top_r: Point,
    bottom_l: Point,
    bottom_r: Point,
    overlaps: bool,
}

impl Rectangle {
    fn new(id: usize, x: usize, y: usize, width: usize, height: usize) -> Self {
        Rectangle {
            id,
            top_l: Point { x, y },
            top_r: Point { x: x + width, y },
            bottom_l: Point { x, y: y + height },
            bottom_r: Point {
                x: x + width,
                y: y + height,
            },
            overlaps: false,
        }
    }

    fn points(&self) -> Points {
        let width = self.top_r.x - self.top_l.x;
        let height = self.bottom_l.y - self.top_l.y;
        Points::new(self.top_l, width, height)
    }

    fn overlaps(&self, r2: &Rectangle) -> bool {
        self.top_l.x < r2.bottom_r.x
            && self.bottom_r.x > r2.top_l.x
            && self.top_l.y < r2.bottom_r.y
            && self.bottom_r.y > r2.top_l.y
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Points {
    top_l: Point,
    max_x: usize,
    max_y: usize,
    x: usize,
    y: usize,
}

impl Points {
    fn new(top_l: Point, width: usize, height: usize) -> Self {
        Points {
            top_l,
            max_x: top_l.x + width - 1,
            max_y: top_l.y + height - 1,
            x: top_l.x,
            y: top_l.y,
        }
    }
}

impl Iterator for Points {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y > self.max_y {
            return None;
        }

        let point = Point {
            x: self.x,
            y: self.y,
        };
        self.x = {
            if self.x == self.max_x {
                self.y += 1;
                self.top_l.x
            } else {
                self.x + 1
            }
        };
        Some(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let r = Rectangle::new(1, 1, 3, 4, 4);
        let r2 = Rectangle::new(2, 3, 1, 4, 4);
        let r3 = Rectangle::new(3, 5, 5, 2, 2);
        assert!(r.overlaps(&r2));
        assert!(r2.overlaps(&r));
        assert!(!r2.overlaps(&r3));
        assert!(!r.overlaps(&r3));
    }
}
