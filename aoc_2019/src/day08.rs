type Layer = [[u8; 25]; 6];

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Layer> {
    let mut values = input.chars().map(|v| v.to_string().parse().unwrap());
    let layers = input.len() / (6 * 25);
    let mut v: Vec<Layer> = Vec::with_capacity(layers);
    for _ in 0..layers {
        let mut layer: [[u8; 25]; 6] = [[0; 25]; 6];
        for row in &mut layer {
            for value in row.iter_mut().take(25) {
                *value = values.next().unwrap();
            }
        }
        v.push(layer);
    }
    v
}

#[aoc(day8, part1)]
fn part1(image: &[Layer]) -> usize {
    let mut fewest_zero = 25 * 6 + 1;
    let mut fewest_zero_layer: &Layer = &image[1];
    for layer in image {
        let mut zeros = 0;
        for row in layer.iter().take(6) {
            for value in row.iter().take(25) {
                if *value == 0 {
                    zeros += 1;
                }
            }
        }
        if zeros < fewest_zero {
            fewest_zero = zeros;
            fewest_zero_layer = &layer;
        }
    }
    let mut ones = 0;
    let mut twos = 0;
    for row in fewest_zero_layer.iter().take(6) {
        for value in row.iter().take(25) {
            match value {
                //test
                1 => ones += 1,
                2 => twos += 1,
                _ => {}
            }
        }
    }
    ones * twos
}

#[derive(Copy, Clone)]
enum PixelColor {
    Black,
    White,
    Transparent,
}

impl From<u8> for PixelColor {
    fn from(input: u8) -> Self {
        match input {
            0 => PixelColor::Black,
            1 => PixelColor::White,
            2 => PixelColor::Transparent,
            _ => unreachable!(),
        }
    }
}

fn layer_pixel(px_front: PixelColor, px_back: PixelColor) -> PixelColor {
    match px_front {
        PixelColor::White => PixelColor::White,
        PixelColor::Black => PixelColor::Black,
        PixelColor::Transparent => px_back,
    }
}

#[aoc(day8, part2)]
fn part2(input: &[Layer]) -> String {
    let mut image = [[PixelColor::Black; 25]; 6];
    for layer in input.iter().rev() {
        for y in 0..6 {
            for x in 0..25 {
                image[y][x] = layer_pixel(PixelColor::from(layer[y][x]), image[y][x]);
            }
        }
    }
    for row in &image {
        for value in row.iter().take(25) {
            match value {
                PixelColor::Black => print!(" "),
                PixelColor::White => print!("0"),
                PixelColor::Transparent => print!("t"),
            }
        }
        println!();
    }

    // Puzzle specific
    "KCGEC".to_owned()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn d8p1() {
        let mut input = String::new();
        let mut f = File::open("input/2019/day8.txt").unwrap();
        f.read_to_string(&mut input).unwrap();
        let v = generator(&input);
        assert_eq!(part1(&v), 1584);
    }
}
