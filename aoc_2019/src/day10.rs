use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Add;
use std::f64::consts::PI;

use ordered_float::NotNan;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn angle(&self, other: &Point) -> f64 {
        let y = (self.y - other.y) as f64;
        let x = (other.x - self.x) as f64;
        let r = x.atan2(y) % (2f64 * PI);
        r
    }
}

fn visible(asteroids: &HashSet<Point>, point: &Point) -> usize {
    let visible: HashSet<_> = asteroids.iter().filter(|x| *x != point).map(|v| NotNan::new(point.angle(v)).unwrap()).collect();
    visible.len()
}

#[aoc_generator(day10)]
fn generator(input: &str) -> HashSet<Point> {
    let mut asteroids = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, value)| {
            if value == '#' {
                asteroids.insert(Point { y: y as isize, x: x as isize });
            }
        })
    });
    asteroids
}

#[aoc(day10, part1)]
fn part1(asteroids: &HashSet<Point>) -> usize {
    asteroids.iter().map(|p| visible(&asteroids, p)).max().unwrap()
}

#[aoc(day10, part2)]
fn part2(asteroids: &HashSet<Point>) -> isize {
    let mut asteroids = asteroids.clone();
    let laser = asteroids.iter().max_by_key(|n| visible(&asteroids, n)).unwrap().clone();
    asteroids.remove(&laser);
    let mut asteroids: Vec<_> = asteroids.into_iter().collect();
    asteroids.sort_by_key(|x| NotNan::new((x.x as f64 - laser.x as f64).hypot(x.y as f64 - laser.y as f64)).unwrap());
    let ranks: HashMap<_, _> = asteroids.iter().cloned().enumerate().map(|(i, b)| {
        (b, asteroids[..i].iter().filter(|c| laser.angle(&b) == laser.angle(c)).count())
    }).collect();
    asteroids.sort_unstable_by(|o, b| ranks.get(b).cmp(&ranks.get(o)).then(laser.angle(b).partial_cmp(&laser.angle(o)).unwrap()));
    let p = asteroids[199];
    (p.x * 100) + p.y
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d10p1() {
        let input = ".#..#\n.....\n#####\n....#\n...##\n";
        let set = generator(input);
        assert_eq!(part1(&set), 8);
        let center = Point{x: 3, y: 2};
        assert_eq!(center.angle(&Point{x: 5, y: 3}), 2f64.atan2(-1f64));
    }
}