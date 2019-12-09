use std::{collections::HashMap, iter};

#[aoc_generator(day6)]
fn generator(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|line| {
            let mut objs = line.split(')');
            let parent = objs.next().unwrap();
            let object = objs.next().unwrap();
            (object.to_owned(), parent.to_owned())
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(orbital_map: &HashMap<String, String>) -> usize {
    let mut num_orbits = 0;
    for object in orbital_map.keys() {
        let mut curr_object = object;
        while let Some(parent) = orbital_map.get(curr_object) {
            num_orbits += 1;
            curr_object = parent;
        }
    }
    num_orbits
}

#[aoc(day6, part2)]
fn part2(orbital_map: &HashMap<String, String>) -> usize {
    let you_ancestors: HashMap<String, usize> =
        iter::successors(orbital_map.get("YOU"), |last| orbital_map.get(&last[..]))
            .enumerate()
            .map(|(i, v)| (v.to_owned(), i))
            .collect();
    let mut steps = 0;
    for ancestor in iter::successors(orbital_map.get("SAN"), |last| orbital_map.get(&last[..])) {
        if you_ancestors.contains_key(ancestor) {
            return steps + *you_ancestors.get(ancestor).unwrap();
        }
        steps += 1;
    }
    0
}
