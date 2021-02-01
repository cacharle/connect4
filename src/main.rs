use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

pub mod position;
pub mod solve;

use position::Position;
use solve::solve;


fn main() {
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
                // println!("{:?}", pos);
                let mut cache = HashMap::with_capacity(30000);
                println!("score: {:3} {:3}", solve(pos, -10000, 10000, &mut cache), expected_score);
            }
            Err(msg) =>
                eprintln!("wrong score format {:?}: {}", fields[1], msg),
        }
    }

    // let mut p = "7422341735647741166133573473242566".parse::<Position>().unwrap();
    // p = p.play(2);
    // p = p.play(2);
    // p = p.play(1);
    // p = p.play(5);
    // println!("{:?}", p);
    // println!("{}", solve(p.clone(), -10000, 100000));
}
