use std::fmt::{self, Display};

#[aoc_generator(day11)]
fn generator(input: &str) -> [[isize; 300]; 300] {
    let mut grid: [[isize; 300]; 300] = [[0; 300]; 300];
    let grid_serial_number: isize = input.parse().unwrap();
    for y in 0..300 {
        for x in 0..300 {
            let rack_id = x as isize + 10;
            let power_level =
                (((((rack_id * (y as isize)) + grid_serial_number) * rack_id) / 100) % 10) - 5;
            grid[y][x] = power_level;
        }
    }
    grid
}

#[aoc(day11, part1)]
fn part1(power_cells: &[[isize; 300]]) -> Square {
    let mut max: isize = 0;
    let mut max_cell = Square {
        x: 0,
        y: 0,
        size: 3,
    };
    for y in 0..298 {
        for x in 0..298 {
            let mut sum = 0;
            for i in 0..3 {
                for j in 0..3 {
                    sum += power_cells[y + i][x + j];
                }
            }
            if sum > max {
                max = sum;
                max_cell.x = x;
                max_cell.y = y;
            }
        }
    }
    max_cell
}

#[aoc(day11, part2)]
fn part2(power_cells: &[[isize; 300]]) -> Square {
    let mut max: isize = 0;
    let mut max_cell = Square {
        x: 0,
        y: 0,
        size: 0,
    };
    for size in 1..=300 {
        for y in 0..=(300 - size) {
            for x in 0..=(300 - size) {
                let mut sum = 0;
                for i in 0..size {
                    for j in 0..size {
                        sum += power_cells[y + i][x + j];
                    }
                }
                if sum > max {
                    max = sum;
                    max_cell.x = x;
                    max_cell.y = y;
                    max_cell.size = size;
                }
            }
        }
    }
    max_cell
}

struct Square {
    x: usize,
    y: usize,
    size: usize,
}

impl Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {}, {})", self.x, self.y, self.size)
    }
}
