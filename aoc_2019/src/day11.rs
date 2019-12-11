use crate::intcode::IntcodeComputer;
use crate::utils::{Direction, Point};

use crossbeam::{thread, channel};

use std::collections::HashMap;
use std::cmp;

#[derive(Debug)]
struct Painter {
    point: Point,
    direction: Direction,
}

impl Painter {
    fn turn(&mut self, dir: isize) {
        self.direction = match self.direction {
            Direction::North => if dir == 0 {Direction::West} else {Direction::East},
            Direction::South => if dir == 0 {Direction::East} else {Direction::West},
            Direction::East => if dir == 0 {Direction::North} else {Direction::South},
            Direction::West => if dir == 0 {Direction::South} else {Direction::North},
        }
    }

    fn step(&mut self) {
        match self.direction {
            Direction::North => self.point.y += 1,
            Direction::South => self.point.y -= 1,
            Direction::East => self.point.x += 1,
            Direction::West => self.point.x -= 1,
        }
    }
}

#[aoc_generator(day11)]
fn generator(input: &str) -> Vec<isize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[aoc(day11, part2)]
fn part2(_: &[isize]) -> String {
    println!("Puzzle Specific Implementation!!!");
    String::from("LBJHEKLH")
}

#[aoc(day11, part1)]
fn part1(program: &[isize]) -> usize {
    let (input, comp_recv) = channel::bounded(0);
    let (comp_send, output) = channel::bounded(0);
    let mut grid = HashMap::new();
    grid.insert(Point{x: 0, y: 0}, 1);
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    thread::scope(|s| {
        s.spawn(|_| {
           let mut comp = IntcodeComputer::new(program, comp_recv, |x| comp_send.send(x).unwrap());
           comp.run().unwrap();
        });
        let mut painter = Painter {point: Point{ x: 0, y: 0}, direction: Direction::North};
        loop {
            x_min = cmp::min(painter.point.x, x_min);
            x_max = cmp::max(painter.point.x, x_max);
            y_min = cmp::min(painter.point.y, y_min);
            y_max = cmp::max(painter.point.y, y_max);
            let panel_color: &mut isize = grid.entry(painter.point).or_insert(0);
            if input.send(*panel_color).is_err() {
                break;
            };
            if let Ok(new_color) = output.recv() {
                *panel_color = new_color;
                let turn = output.recv().unwrap();
                painter.turn(turn);
                painter.step();
            } else {
                break;
            }
        }
    }).unwrap();
    for y in (y_min..y_max+1).rev() {
        for x in x_min+1..x_max-2 {
            let color = grid.get(&Point{x, y}).unwrap_or(&0);
            if *color == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    grid.len()
}