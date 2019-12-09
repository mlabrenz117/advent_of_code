#[aoc_generator(day4)]
fn generator(input: &str) -> (usize, usize) {
    let mut range = input.split('-').map(|s| s.parse::<usize>().unwrap());
    (range.next().unwrap(), range.next().unwrap())
}

#[aoc(day4, part1)]
fn part1(input: &(usize, usize)) -> usize {
    (input.0..input.1)
        .filter(|x| monotonic(*x))
        .filter(|x| has_multiple(*x))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &(usize, usize)) -> usize {
    (input.0..input.1)
        .filter(|x| monotonic(*x))
        .filter(|x| has_double(*x))
        .count()
}

fn monotonic(value: usize) -> bool {
    let mut v = value;
    let mut last = 9;
    while v > 0 {
        if v % 10 > last {
            return false;
        }
        last = v % 10;
        v /= 10;
    }
    true
}

fn has_multiple(value: usize) -> bool {
    let mut last: isize = -1;
    let mut v = value;
    while v > 0 {
        if v as isize % 10 == last {
            return true;
        }
        last = v as isize % 10;
        v /= 10;
    }
    false
}

fn has_double(value: usize) -> bool {
    let s = value.to_string();
    let mut v = Vec::with_capacity(s.len());
    for c in s.chars() {
        if let Some(&l) = v.last() {
            if c == l {
                v.push(c)
            } else if v.len() == 2 {
                return true;
            } else {
                v.clear();
                v.push(c);
            }
        } else {
            v.push(c);
        }
    }
    v.len() == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d4() {
        assert!(has_double(112233));
        assert!(!has_double(123444));
        assert!(has_double(111122));
    }
}
