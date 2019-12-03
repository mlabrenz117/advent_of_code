use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Display;

lazy_static! {
    static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
}

#[aoc_generator(day6)]
fn generator(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|x| {
            let matches: Vec<usize> = RE
                .find_iter(x)
                .map(|x| x.as_str().parse().unwrap())
                .collect();
            Point(matches[0], matches[1])
        })
        .collect()
}

#[aoc(day6, part1)]
fn one(grid: &[Point]) -> usize {
    let grid = Grid::new(grid);
    grid.unsafe_area()
}

#[aoc(day6, part2)]
fn two(grid: &[Point]) -> usize {
    let grid = Grid::new(grid);
    grid.safe_area()
}

struct Grid<'a> {
    points: &'a [Point],
    boundries: [usize; 4],
    grid: Vec<Vec<(Option<&'a Point>, usize)>>,
}

impl<'a> Grid<'a> {
    fn new(points: &[Point]) -> Grid {
        let (boundries, grid) = Grid::calculate(points);
        Grid {
            boundries,
            grid,
            points,
        }
    }

    fn find_boundries(points: &[Point]) -> [usize; 4] {
        use std::usize::{MAX, MIN};
        let mut boundries = [MAX, MIN, MAX, MIN]; // Top, Bottom, Left, Right
        for point in points {
            if point.1 < boundries[0] {
                boundries[0] = point.1 - 1;
            }
            if point.1 > boundries[1] {
                boundries[1] = point.1 + 1;
            }
            if point.0 < boundries[2] {
                boundries[2] = point.0 - 1;
            }
            if point.0 > boundries[3] {
                boundries[3] = point.0 + 1;
            }
        }
        boundries
    }

    fn unsafe_area(&self) -> usize {
        self.points
            .iter()
            .filter(|p| !p.escapes(self))
            .map(|point| {
                self.grid
                    .iter()
                    .flatten()
                    .filter(|(p, _)| if let Some(p) = p { p == &point } else { false })
                    .count()
            })
            .max()
            .expect("An error occured")
    }

    fn safe_area(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|(_, sum)| *sum < 10000)
            .count()
    }

    fn calculate(points: &[Point]) -> ([usize; 4], Vec<Vec<(Option<&Point>, usize)>>) {
        let boundries = Grid::find_boundries(points);
        let mut grid = Vec::with_capacity(boundries[1] - boundries[0] + 1);
        for y in boundries[0]..=boundries[1] {
            let mut row = Vec::with_capacity(boundries[3] - boundries[2] + 1);
            for x in boundries[2]..=boundries[3] {
                let p = Point(x, y);
                row.push({
                    let mut distances = HashMap::new();
                    points
                        .iter()
                        .map(|point| (point, point.manhattan_distance(&p)))
                        .for_each(|(i, d)| (*distances.entry(d).or_insert(Vec::new())).push(i));

                    let sum = distances
                        .iter()
                        .fold(0, |acc, (dist, points)| acc + dist * points.len());

                    let closest_points = distances.iter().min_by_key(|(&d, _)| d).unwrap();
                    if closest_points.1.len() == 1 {
                        (Some(closest_points.1[0]), sum)
                    } else {
                        (None, sum)
                    }
                });
            }
            grid.push(row);
        }
        (boundries, grid)
    }
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point(usize, usize);

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        (((self.0 as isize) - (other.0 as isize)).abs()
            + ((self.1 as isize) - (other.1 as isize)).abs()) as usize
    }

    fn escapes(&self, grid: &Grid) -> bool {
        let x = self.0 - grid.boundries[2];
        let y = self.1 - grid.boundries[0];

        if let (Some(p), _) = grid.grid[0][x] {
            if p == self {
                return true;
            }
        }
        if let (Some(p), _) = grid.grid[y][grid.boundries[3] - grid.boundries[2]] {
            if p == self {
                return true;
            }
        }
        if let (Some(p), _) = grid.grid[grid.boundries[1] - grid.boundries[0]][x] {
            if p == self {
                return true;
            }
        }
        if let (Some(p), _) = grid.grid[y][0] {
            if p == self {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let points = vec![
            Point(1, 1),
            Point(1, 6),
            Point(8, 3),
            Point(3, 4),
            Point(5, 5),
            Point(8, 9),
        ];
        let grid = Grid::new(&points);
        println!("{}", grid);
        println!("{:?}", grid.unsafe_area());
    }
}

impl Display for Grid<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hash: HashMap<&Point, usize> = self
            .points
            .iter()
            .enumerate()
            .map(|(v, k)| (k, v + 1))
            .collect();
        for row in self.grid.iter() {
            for s in row.iter().map(|point| match point {
                (Some(k), _) => hash.get(k).unwrap().to_string(),
                (None, _) => ".".to_string(),
            }) {
                write!(fmt, "{}", s)?;
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}
