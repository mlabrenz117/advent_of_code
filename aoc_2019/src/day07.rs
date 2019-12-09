use std::sync::mpsc::sync_channel;

use crate::intcode::*;

use crossbeam::thread;
use itertools::Itertools;

#[aoc_generator(day7)]
fn generator(input: &str) -> Vec<isize> {
    input.split(',').map(|v| v.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(program: &[isize]) -> isize {
    let (send, recv) = sync_channel(120);
    thread::scope(|s| {
        for phase_setting in (0..=4).permutations(5) {
            let sender = send.clone();
            s.spawn(move |_| {
                let mut in_access = vec![0, 0, 0, 0, 0];
                let mut out: isize = 0;
                let out_ptr = &mut out as *mut isize;
                let mut out_fn = |v| unsafe {
                    *out_ptr = v;
                };
                let mut in1 = || get_input(0, &mut in_access[..], &phase_setting[..], out);
                let mut amp1 = IntcodeComputer::new(program, &mut in1, &mut out_fn);
                amp1.run();
                let mut in2 = || get_input(1, &mut in_access[..], &phase_setting[..], out);
                let mut amp2 = IntcodeComputer::new(program, &mut in2, &mut out_fn);
                amp2.run();
                let mut in3 = || get_input(2, &mut in_access[..], &phase_setting[..], out);
                let mut amp3 = IntcodeComputer::new(program, &mut in3, &mut out_fn);
                amp3.run();
                let mut in4 = || get_input(3, &mut in_access[..], &phase_setting[..], out);
                let mut amp4 = IntcodeComputer::new(program, &mut in4, &mut out_fn);
                amp4.run();
                let mut in5 = || get_input(4, &mut in_access[..], &phase_setting[..], out);
                let mut amp5 = IntcodeComputer::new(program, &mut in5, &mut out_fn);
                amp5.run();
                sender.send(out).unwrap();
            });
        }
    }).unwrap();
    drop(send);
    //let r : Vec<_> = recv.iter().collect();
    recv.iter().max().unwrap()
}

#[aoc(day7, part2)]
fn part2(program: &[isize]) -> isize {
    let (send, recv) = sync_channel(128);
    thread::scope(|s| {
       for phase_setting in (5..=9).permutations(5) {
           let mut out = 0;
           let sender = send.clone();
           let (a_s, a_r) = sync_channel(32);
           let (b_s, b_r) = sync_channel(32);
           let (c_s, c_r) = sync_channel(32);
           let (d_s, d_r) = sync_channel(32);
           let (e_s, e_r) = sync_channel(32);
           a_s.send(phase_setting[0]).unwrap();
           b_s.send(phase_setting[1]).unwrap();
           c_s.send(phase_setting[2]).unwrap();
           d_s.send(phase_setting[3]).unwrap();
           e_s.send(phase_setting[4]).unwrap();
           a_s.send(0).unwrap();
           s.spawn(move |_| {
               let mut in_fn = || a_r.recv().unwrap();
               let mut out_fn = |x| b_s.send(x).unwrap();
               let mut ic = IntcodeComputer::new(&program, &mut in_fn, &mut out_fn);
               ic.run();
           });
           s.spawn(move |_| {
               let mut in_fn = || b_r.recv().unwrap();
               let mut out_fn = |x| c_s.send(x).unwrap();
               let mut ic = IntcodeComputer::new(&program, &mut in_fn, &mut out_fn);
               ic.run();
           });
           s.spawn(move |_| {
               let mut in_fn = || c_r.recv().unwrap();
               let mut out_fn = |x| d_s.send(x).unwrap();
               let mut ic = IntcodeComputer::new(&program, &mut in_fn, &mut out_fn);
               ic.run();
           });
           s.spawn(move |_| {
               let mut in_fn = || d_r.recv().unwrap();
               let mut out_fn = |x| e_s.send(x).unwrap();
               let mut ic = IntcodeComputer::new(&program, &mut in_fn, &mut out_fn);
               ic.run();
           });
           s.spawn(move |_| {
               let mut in_fn = || e_r.recv().unwrap();
               let mut out_fn = |x| {
                   if let Err(_) = a_s.send(x) {
                       out = x;
                   }
               };
               let mut ic = IntcodeComputer::new(&program, &mut in_fn, &mut out_fn);
               ic.run();
               sender.send(out).unwrap();
           });
       }
    }).unwrap();
    drop(send);
    recv.iter().max().unwrap()
}

fn get_input(n: usize, in_access: &mut [usize], values: &[usize], out: isize) -> isize {
    if in_access[n] == 0 {
        in_access[n] = 1;
        values[n] as isize
    } else {
        out
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
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let v = generator(input);
        let a2 = part2(&v);
        assert_eq!(a2, 139629729);
    }
}
