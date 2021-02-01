use std::collections::HashMap;

use crate::position::{Position, WIDTH, HEIGHT};


pub fn solve(p: Position, a: i32, b: i32, cache: &mut HashMap<u64, i32>) -> i32 {
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

    for x in 0..WIDTH {
        if !p.is_valid_play(x) {
            continue
        }

        let score = -solve(p.play(x), -beta, -alpha, cache);

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

