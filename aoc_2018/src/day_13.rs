use std::{fmt::{self, Display}, collections::{HashMap, HashSet}};
use itertools::Itertools;

#[derive(Debug)]
struct DataStore {
    track: Vec<Vec<Option<Track>>>,
    carts: Vec<Cart>,
}

#[derive(Debug, Clone)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    turn: Turn,
}

#[derive(Debug, Clone, Copy)]
enum Track {
    Horizontal,
    Vertical,
    Intersection,
    CurveLeft,
    CurveRight,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[aoc_generator(day13)]
fn generator(input: &str) -> Box<DataStore> {
    let mut puzzle_input = Box::new(DataStore {
        track: Vec::new(),
        carts: Vec::new(),
    });
    
    input.lines().enumerate().for_each(|(y, line)| {
        puzzle_input.track.push(Vec::new());
       line.chars().map(parse).enumerate().for_each(|(x, cell)| {
           puzzle_input.track[y].push(cell.1);
           if let Some(direction) = cell.0 {
               puzzle_input.carts.push(Cart {x, y, direction, turn: Turn::Left});
           }
       })
    });
    puzzle_input
}

#[aoc(day13, part1)]
fn first_collision(data: &DataStore) -> Cart {
    let mut carts = data.carts.clone();
    let mut locations = HashSet::new();
    loop {
        for cart in carts.iter_mut().sorted_by_key(|c| c.y) {
            cart.step();
            if !locations.insert((cart.x, cart.y)) {
                return cart.clone();
            }
            let track = data.track[cart.y][cart.x].expect("Carts flew off the track");
            cart.turn(track);
        }
        locations.clear();
    }
}

#[aoc(day13, part2)]
fn last_cart(data: &DataStore) -> Cart {
    let mut carts = data.carts.clone();
    let mut locations = HashMap::new();
    let mut collisions = Vec::new();
    loop {
        if carts.len() == 1 {
            return carts[0].clone();
        }
        for (i, cart) in carts.iter_mut().enumerate().sorted_by(|(_, a), (_, b)| a.y.cmp(&b.y).then(a.x.cmp(&b.x))) {
            cart.step();
            if let Some(j) = locations.insert((cart.x, cart.y), i) {
                collisions.push(i);
                collisions.push(j);
                locations.remove(&(cart.x, cart.y));
            }
            let track = data.track[cart.y][cart.x].expect("Cart flew off the track");
            cart.turn(track);
        }

        #[cfg(test)]
        for y in 0..data.track.len() {
            for x in 0..data.track[y].len() {
                if let Some(idx) = locations.get(&(x, y)) {
                    let c: char = carts[*idx].direction.into();
                    print!("{}", c);
                } else {
                    let c: char = data.track[y][x].map(|t| t.into()).unwrap_or(' ');
                    print!("{}", c);
                }
            }
            println!();
        }

        collisions.sort_by_key(|i| -(*i as isize));
        for i in collisions.drain(..) {
            carts.remove(i);
        }
        locations.clear();
    }
}

fn parse(cell: char) -> (Option<Direction>, Option<Track>) {
    match cell {
        '^' => (Some(Direction::North), Some(Track::Vertical)),
        '>' => (Some(Direction::East), Some(Track::Horizontal)),
        'v' => (Some(Direction::South), Some(Track::Vertical)),
        '<' => (Some(Direction::West), Some(Track::Horizontal)),
        '\\' => (None, Some(Track::CurveLeft)),
        '/' => (None, Some(Track::CurveRight)),
        '-' => (None, Some(Track::Horizontal)),
        '|' => (None, Some(Track::Vertical)),
        '+' => (None, Some(Track::Intersection)),
        _ => (None, None),
    }
}

impl Cart {
    fn step(&mut self) {
        match self.direction {
            Direction::South => {
                self.y += 1;
            }
            Direction::North => {
                self.y -= 1;
            }
            Direction::West => {
                self.x -= 1;
            }
            Direction::East => {
                self.x += 1;
            }
        }
    }

    fn turn(&mut self, track: Track) {
        self.direction = match track {
            Track::CurveRight => self.direction.turn(self.direction.map_curve(Track::CurveRight)),
            Track::CurveLeft => self.direction.turn(self.direction.map_curve(Track::CurveLeft)),
            Track::Intersection => {
                let next_dir = self.direction.turn(self.turn);
                self.turn = self.next_turn();
                next_dir
            }
            _ => self.direction,
        }
    }

    fn next_turn(&self) -> Turn {
        match self.turn {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

impl Display for Cart {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        if let Turn::Straight = turn {
            return *self;
        }
        match self {
            Direction::North => {
                match turn {
                    Turn::Left => Direction::West,
                    Turn::Right => Direction::East,
                    _ => unreachable!(),
                }
            }
            Direction::South => {
                match turn {
                    Turn::Left => Direction::East,
                    Turn::Right => Direction::West,
                    _ => unreachable!(),
                }
            }
            Direction::West => {
                match turn {
                    Turn::Left => Direction::South,
                    Turn::Right => Direction::North,
                    _ => unreachable!(),
                }
            }
            Direction::East => {
                match turn {
                    Turn::Left => Direction::North,
                    Turn::Right => Direction::South,
                    _ => unreachable!(),
                }
            }
        }
    }

    fn map_curve(&self, t: Track) -> Turn {
        match t {
            Track::CurveLeft => {
                match self {
                    Direction::North | Direction::South => Turn::Left,
                    _ => Turn::Right,
                }
            }
            Track::CurveRight => {
                match self {
                    Direction::North | Direction::South => Turn::Right,
                    _ => Turn::Left,

                }
            }
            _ => Turn::Straight,
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::West => '<',
            Direction::East => '>',
        }
    }
}

impl Into<char> for Track {
    fn into(self) -> char {
        match self {
            Track::CurveLeft => '\\',
            Track::CurveRight => '/',
            Track::Intersection => '+',
            Track::Horizontal => '-',
            Track::Vertical => '|',
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};
    use super::*;
    #[test]
    fn test() {
        let mut f = File::open("./input/2018/day13_test.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let input = generator(&s);
        let cart = first_collision(&input);
        assert_eq!(cart.x, 7);
        assert_eq!(cart.y, 3);
    }

    #[test]
    fn test2() {
        let mut f = File::open("./input/2018/day13_test2.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let input = generator(&s);
        let cart = last_cart(&input);
        dbg!(&cart);
        assert_eq!(cart.x, 6);
        assert_eq!(cart.y, 4);
        assert!(false);
    }
}
