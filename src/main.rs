use std::io;
use std::io::prelude::*;
use std::time::{Instant,Duration};

pub mod position;
pub mod solve;

use position::Position;
use solve::solve;


fn main() {
    let mut total_time = Duration::new(0, 0);
    let mut total_solve = 0;

    for result in  io::stdin().lock().lines() {
        let line = result.unwrap();
        let fields: Vec<&str> = line.split_ascii_whitespace().collect();
        if fields.len() != 2 {
            eprintln!("wrong line format {:?}", line);
            continue
        }
        let expected_score = match fields[1].parse::<i32>() {
            Ok(n) => n,
            Err(msg) => {
                eprintln!("wrong score format {:?}: {}", fields[1], msg);
                continue;
            }
        };
        match fields[0].parse::<Position>() {
            Ok(pos) => {
                print!("{:?}", pos);
                let begin = Instant::now();
                let score = solve(pos);
                let elapsed = begin.elapsed();
                total_time += elapsed;
                total_solve += 1;
                println!("score: {:3} {:3}, time: {:?}\n", score, expected_score, elapsed);
            }
            Err(msg) =>
                eprintln!("wrong score format {:?}: {}", fields[1], msg),
        }
    }
    println!("mean time {:?}", total_time / total_solve);
}
