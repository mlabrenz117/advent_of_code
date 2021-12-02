use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .fold((0, usize::MAX), |(num_descend, prev), next| {
            if *next > prev {
                (num_descend + 1, *next)
            } else {
                (num_descend, *next)
            }
        })
        .0
}

#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut sliding_window = SlidingWindow::new(input, 3);
    sliding_window
        .fold((0, usize::MAX), |(num_descend, prev), next| {
            let sum = next.iter().sum();
            if sum > prev {
                (num_descend + 1, sum)
            } else {
                (num_descend, sum)
            }
        })
        .0
}

pub struct SlidingWindow<'a> {
    data: &'a [usize],
    window_size: usize,
    cursor: usize,
}

impl<'a> SlidingWindow<'a> {
    pub fn new(data: &'a [usize], window_size: usize) -> Self {
        Self {
            data,
            window_size,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for SlidingWindow<'a> {
    type Item = &'a [usize];

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.cursor;
        let end = start + self.window_size;
        if end > self.data.len() {
            return None;
        }
        self.cursor += 1;
        Some((&self.data[start..end]))
    }
}
