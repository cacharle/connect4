use std::collections::HashMap;

use crate::position::{Position, WIDTH, HEIGHT, MIN_SCORE};

const COLUMNS_ORDER: [u64; 7] = [3, 2, 4, 1, 5, 0, 6];

pub fn solve(p: Position) -> i32 {
    let mut cache = HashMap::with_capacity(30000);
    solve_rec(p, -1000, 1000, &mut cache)
}

// the weak solver only tells if the position is a win/lose/draw
// it's faster but less precise
pub fn solve_weak(p: Position) -> i32 {
    let mut cache = HashMap::with_capacity(30000);
    solve_rec(p, -1, 1, &mut cache)
}

fn solve_rec(p: Position, a: i32, b: i32, cache: &mut HashMap<u64, i32>) -> i32 {
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

    if let Some(max_score) = cache.get(&p.key()) {
        // can't return max_score directly
        // because the alpha-beta context in the cache may be
        // different than the current alpha-beta
        if beta > *max_score {
            beta = *max_score;
            if alpha >= beta {
                return beta;
            }
        }
    }

    let mut best = MIN_SCORE;
    for x in (0..(WIDTH as usize))
                .map(|x| COLUMNS_ORDER[x])
                .filter(|x| p.is_valid_play(*x))
    {
        // using negamax, variante of minimax where:
        // max(player1, player2) == -min(-player1, -player2)
        let score = -solve_rec(p.play(x), -beta, -alpha, cache);
        if score > best {
            best = score;
        }
        // reduce alpha-beta range if found better score
        if best > alpha {
            alpha = best;
        }
        // impossible alpha-beta range reached (alpha is supposed to be < to beta)
        if alpha >= beta {
            return score;
        }
    }
    cache.insert(p.key(), best);
    return best;
}

