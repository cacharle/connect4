use std::collections::HashMap;

use crate::position::{Position, WIDTH, HEIGHT};

const COLUMNS_ORDER: [u64; 7] = [3, 2, 4, 1, 5, 0, 6];

pub fn solve(p: Position) -> i32 {
    let mut cache = HashMap::with_capacity(30000);
    solve_rec(p, -100000, 100000, &mut cache)
}

fn solve_rec(p: Position, a: i32, b: i32, cache: &mut HashMap<u64, i32>) -> i32 {
    if let Some(score) = cache.get(&p.key()) {
        return *score;
    }
    if p.is_draw() {
        return 0;
    }
    for x in 0..WIDTH {
        if p.is_valid_play(x) && p.is_winning_play(x) {
            return (((WIDTH * HEIGHT + 1) as i32) - (p.play_count as i32)) / 2;
        }
    }

    let mut alpha = a;
    let mut beta = b;

    let max = (((WIDTH * HEIGHT + 1) as i32) - (p.play_count as i32)) / 2;

    if beta > max {
        beta = max;
        if alpha >= beta {
            return beta;
        }
    }

    for x_unordered in 0..(WIDTH as usize) {
        let x = COLUMNS_ORDER[x_unordered];
        if !p.is_valid_play(x) {
            continue
        }

        let score = -solve_rec(p.play(x), -beta, -alpha, cache);

        if score >= beta {
            return score;
        }

        if score > alpha {
            alpha = score;
        }
    }
    cache.insert(p.key(), alpha);
    return alpha;
}

