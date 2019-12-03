use chrono::{NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE_NUM: Regex = Regex::new(r"(?P<guard>\d+)").unwrap();
    static ref RE: Regex = Regex::new(r"\[(.{16})\] (.*)$").unwrap();
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Event> {
    struct Inner<'a> {
        dt: NaiveDateTime,
        s: &'a str,
    }
    let mut v: Vec<_> = input
        .lines()
        .map(|line| {
            let caps = RE.captures(line).unwrap();
            Inner {
                dt: NaiveDateTime::parse_from_str(caps.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M")
                    .expect("Could not parse date"),
                s: caps.get(2).unwrap().as_str(),
            }
        })
        .collect();
    v.sort_by(|a, b| a.dt.cmp(&b.dt));
    let mut last_id = 0;
    v.into_iter()
        .map(|i| {
            let action = Action::parse_str(i.s);
            if let Action::BeginShift(id) = action {
                last_id = id;
            }
            Event {
                id: last_id,
                dt: i.dt,
                action,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn one(input: &[Event]) -> u64 {
    let mut map = HashMap::new();
    let mut curr_id = 0;
    let mut sleep_time = 0;
    for event in input {
        match event.action {
            Action::BeginShift(id) => {
                curr_id = id;
            }
            Action::FallAsleep => {
                sleep_time = u64::from(event.dt.minute());
            }
            Action::WakeUp => {
                let time_asleep = u64::from(event.dt.minute()) - sleep_time;
                map.entry(curr_id).or_insert(Vec::new()).push(time_asleep);
            }
        }
    }

    let max_id = map
        .iter()
        .map(|(k, v)| (k, v.iter().sum::<u64>()))
        .max_by_key(|(_, y)| y.clone())
        .unwrap()
        .0;

    let mut times = HashMap::new();
    let mut t = 0;
    input
        .iter()
        .filter(|x| x.id == *max_id)
        .for_each(|x| match x.action {
            Action::FallAsleep => t = x.dt.minute(),
            Action::WakeUp => {
                for i in t..x.dt.minute() {
                    *times.entry(i).or_insert(0) += 1;
                }
            }
            _ => {}
        });
    let max_time = times.iter().max_by_key(|(_, y)| y.clone()).unwrap().0;

    *max_id as u64 * *max_time as u64
}

#[aoc(day4, part2)]
fn two(input: &[Event]) -> usize {
    let mut mins: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 60];
    let mut t = 0;
    input
        .iter()
        .filter(|x| x.action == Action::FallAsleep || x.action == Action::WakeUp)
        .for_each(|event| match event.action {
            Action::FallAsleep => t = event.dt.minute(),
            Action::WakeUp => {
                for min in t..event.dt.minute() {
                    *mins
                        .get_mut(min as usize)
                        .unwrap()
                        .entry(event.id)
                        .or_insert(0) += 1;
                }
            }
            _ => {}
        });
    let ans = mins
        .iter()
        .enumerate()
        .filter_map(|(i, hashmap)| {
            let v = hashmap.iter().max_by_key(|(_, &t)| t);
            if let Some(tuple) = v {
                Some((i, tuple))
            } else {
                None
            }
        })
        .max_by_key(|(_, (_, &t))| t)
        .expect("One");
    ans.0 * (ans.1).0
}

#[derive(Debug)]
pub struct Event {
    id: usize,
    dt: NaiveDateTime,
    action: Action,
}

#[derive(Debug, PartialEq)]
enum Action {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

impl Action {
    fn parse_str(input: &str) -> Self {
        match input {
            "falls asleep" => Action::FallAsleep,
            "wakes up" => Action::WakeUp,
            _s => {
                let id = RE_NUM.captures(_s).unwrap()["guard"]
                    .parse::<usize>()
                    .unwrap();
                Action::BeginShift(id)
            }
        }
    }
}
