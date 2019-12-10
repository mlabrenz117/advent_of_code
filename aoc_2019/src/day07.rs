use std::{collections::VecDeque, iter};

use crate::intcode::*;

use crossbeam::{channel, thread};
use itertools::Itertools;

#[aoc_generator(day7)]
fn generator(input: &str) -> Vec<isize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(program: &[isize]) -> isize {
    let (send, recv) = channel::bounded(120);
    thread::scope(|s| {
        for phase_setting in (0..=4).permutations(5) {
            let sender = send.clone();
            s.spawn(move |_| {
                let mut in_access = vec![0, 0, 0, 0, 0];
                let mut out: isize = 0;
                let out_ptr = &mut out as *mut isize;
                let out_fn = |v| unsafe {
                    *out_ptr = v;
                };
                for i in 0..5 {
                    let input =
                        iter::from_fn(|| get_input(i, &mut in_access[..], &phase_setting[..], out));
                    let mut amp = IntcodeComputer::new(program, input, out_fn);
                    amp.run().unwrap();
                }
                sender.send(out).unwrap();
            });
        }
    })
    .unwrap();
    drop(send);
    recv.iter().max().unwrap()
}

#[aoc(day7, part2)]
fn part2(program: &[isize]) -> isize {
    let (send, recv) = channel::bounded(128);
    thread::scope(|s| {
        for phase_setting in (5..=9).permutations(5) {
            const NUM_AMPS: usize = 5;
            let mut amp_senders = VecDeque::with_capacity(5);
            let mut amp_receivers = VecDeque::with_capacity(5);
            for phase in phase_setting.iter().take(NUM_AMPS) {
                let phase = *phase as isize;
                let (s, r) = channel::bounded(8);
                s.send(phase).unwrap();
                amp_senders.push_back(s);
                amp_receivers.push_back(r);
            }
            let a_sender = amp_senders.pop_front().unwrap();
            a_sender.send(0).unwrap();
            amp_senders.push_back(a_sender);
            for i in 0..NUM_AMPS {
                let sender = amp_senders.pop_front().unwrap();
                let receiver = amp_receivers.pop_front().unwrap();
                let result_sender = send.clone();
                s.spawn(move |_| {
                    let out_fn = |x| {
                        if sender.send(x).is_err() && i == NUM_AMPS - 1 {
                            // The first amplifier has halted
                            result_sender.send(x).unwrap();
                        }
                    };
                    let mut ic = IntcodeComputer::new(&program, receiver, out_fn);
                    ic.run().unwrap();
                });
            }
        }
    })
    .unwrap();
    drop(send);
    recv.iter().max().unwrap()
}

fn get_input(n: usize, in_access: &mut [usize], values: &[usize], out: isize) -> Option<isize> {
    if in_access[n] == 0 {
        in_access[n] = 1;
        Some(values[n] as isize)
    } else {
        Some(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d7p1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let v = generator(input);
        let a1 = part1(&v);
        assert_eq!(a1, 43210);
    }

    #[test]
    fn d7p2() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let v = generator(input);
        let a2 = part2(&v);
        assert_eq!(a2, 139629729);
    }
}
