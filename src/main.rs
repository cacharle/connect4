use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};

pub mod position;
pub mod solver;

use position::Position;
use separator::Separatable;
use solver::Solver;

fn main() -> Result<(), Box<dyn Error>> {
    let mut total_time = Duration::new(0, 0);
    let mut total_solve = 0;
    let mut total_visited = 0;
    let mut solver = Solver::new();

    for result in io::stdin().lock().lines() {
        let line = result?;
        let fields: Vec<&str> = line.split_ascii_whitespace().collect();
        if fields.len() != 2 {
            eprintln!("wrong line format {:?}", line);
            continue;
        }
        let expected_score = fields[1].parse::<i32>()?;
        let pos = fields[0].parse::<Position>()?;
        // print!("{:?}", pos);
        let begin = Instant::now();
        let score = solver.solve(pos);
        let elapsed = begin.elapsed();
        println!(
            "{:03}: score: {:3}, time: {:>8.2?}, visited {:>10}",
            total_solve,
            score,
            elapsed,
            solver.visited.separated_string()
        );
        if score != expected_score {
            eprintln!(
                "{:03}: score: {:3} {:3}",
                total_solve, score, expected_score
            );
        }
        assert_eq!(score, expected_score);
        total_time += elapsed;
        total_solve += 1;
        total_visited += solver.visited;
        solver.reset();
    }
    println!(
        "mean time: {:?} | mean visited {:10}",
        total_time / total_solve,
        (total_visited / total_solve as usize).separated_string()
    );
    Ok(())
}
