use std::cmp;
const DIFF: u8 = b'a' - b'A';

#[inline]
fn diff(c1: u8, c2: u8) -> bool {
    cmp::max(c1, c2) - cmp::min(c1, c2) == DIFF
}

#[aoc(day5, part1)]
fn stack(input: &str) -> usize {
    let mut stack = Vec::new();
    for c in input.chars() {
        if let Some(last) = stack.last() {
            if diff(*last as u8, c as u8) {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }
    stack.len()
}

#[aoc(day5, part2)]
fn two(input: &str) -> usize {
    let letters: Vec<(char, char)> = (65..91)
        .map(char::from)
        .zip((97..123).map(char::from))
        .collect();
    let mut collapsed_lens = Vec::new();
    for letter in letters {
        let s = input.replace(letter.0, ""); // The two copies here
        let s = s.replace(letter.1, ""); // and here are sad
        collapsed_lens.push(stack(&s));
    }

    collapsed_lens.into_iter().min().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let s = "lDdhGghHHLAXxaKkFfDdsS";
        assert!(diff('c' as u8, 'C' as u8));
        assert!(diff('C' as u8, 'c' as u8));
        assert!(!diff('A' as u8, 'c' as u8));
        assert!(!diff('c' as u8, 'A' as u8));
        assert!(!diff('A' as u8, 'A' as u8));
        assert!(!diff('c' as u8, 'c' as u8));
        stack(&s);
    }

    #[test]
    fn other() {
        let s = "DdabAcCaCBxAcCcaACDAa";
        assert_eq!(7, stack(&s));
    }
}
