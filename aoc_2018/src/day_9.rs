use linked_list::{Cursor, LinkedList};

#[aoc_generator(day9)]
fn generator(input: &str) -> Box<(usize, usize)> {
    let mut input = input.split_whitespace();
    Box::new((
        input.next().unwrap().parse().unwrap(),
        input.nth(5).unwrap().parse().unwrap(),
    ))
}

#[aoc(day9, part1)]
fn part1(input: &(usize, usize)) -> usize {
    let (players, num_marbles) = *input;
    let mut list = LinkedList::new();
    let mut points = vec![0; players];
    let mut marbles = MarbleRing::new(&mut list);
    for marble in 0..=num_marbles {
        if marble != 0 && marble % 23 == 0 {
            points[marble % players] += marble;
            points[marble % players] += marbles.remove_marble()
        } else {
            marbles.add_marble(marble);
        }
    }
    *points.iter().max().unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &(usize, usize)) -> usize {
    part1(&(input.0, input.1 * 100))
}

struct MarbleRing<'a> {
    len: usize,
    cursor: Cursor<'a, usize>,
}

impl<'a> MarbleRing<'a> {
    fn new(list: &'a mut LinkedList<usize>) -> Self {
        Self {
            len: 0,
            cursor: list.cursor(),
        }
    }

    fn add_marble(&mut self, marble: usize) {
        if self.len == 0 {
            self.cursor.insert(marble);
        } else {
            let mut a = false;
            for _ in 0..2 {
                a |= self.cursor.next().is_none();
            }
            if a {
                self.cursor.next();
            }

            self.cursor.insert(marble);
        }
        self.len += 1;
    }

    fn remove_marble(&mut self) -> usize {
        let mut a = false;
        for _ in 0..7 {
            a |= self.cursor.prev().is_none();
        }
        if a {
            self.cursor.prev();
        }
        self.len -= 1;
        self.cursor.remove().unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn d9p1() {
        assert_eq!(32, super::part1(&(9, 25)));
        assert_eq!(8317, super::part1(&(10, 1618)));
    }
}
