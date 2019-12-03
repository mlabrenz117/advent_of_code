use std::collections::HashSet;

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<Star> {
    input
        .lines()
        .map(|line| Star {
            position: Vector {
                x: line[10..16].trim_start().parse().expect("1"),
                y: line[18..24].trim_start().parse().expect("2"),
            },
            velocity: Vector {
                x: line[36..38].trim_start().parse().expect("3"),
                y: line[40..42].trim_start().parse().expect("4"),
            },
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Star]) -> usize {
    let mut stars: Vec<Star> = input.iter().cloned().collect();

    let mut map = HashSet::new();

    let mut y_min = stars.iter().map(|s| s.position.y).min().unwrap();
    let mut y_max = stars.iter().map(|s| s.position.y).max().unwrap();
    let mut x_min = stars.iter().map(|s| s.position.x).min().unwrap();
    let mut x_max = stars.iter().map(|s| s.position.x).max().unwrap();

    let mut area = (x_max - x_min) * (y_max - y_min);
    let mut new_area = area;

    let mut i = 0;
    while new_area <= area {
        area = new_area;
        for star in stars.iter_mut() {
            star.step();
        }
        y_min = stars.iter().map(|s| s.position.y).min().unwrap();
        y_max = stars.iter().map(|s| s.position.y).max().unwrap();
        x_min = stars.iter().map(|s| s.position.x).min().unwrap();
        x_max = stars.iter().map(|s| s.position.x).max().unwrap();
        new_area = (x_max - x_min) * (y_max - y_min);
        i += 1;
    }

    for star in stars.iter_mut() {
        star.leap(-1);
        map.insert((star.position.x, star.position.y));
    }

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    i - 1
}

#[derive(Copy, Clone, Debug)]
struct Star {
    velocity: Vector,
    position: Vector,
}

impl Star {
    fn step(&mut self) {
        self.position += self.velocity;
    }

    fn leap(&mut self, n: isize) {
        self.position += &self.velocity * n;
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Vector {
    x: isize,
    y: isize,
}

impl std::ops::AddAssign<Self> for Vector {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign<Self> for Vector {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Mul<isize> for &Vector {
    type Output = Vector;

    fn mul(self, other: isize) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
