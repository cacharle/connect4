use fnv::FnvHashMap;  // better hashing function performance on small key (e.g u64)

use crate::position::{Position, WIDTH, HEIGHT, MIN_SCORE};


const COLUMNS_ORDER: [u64; 7] = [3, 2, 4, 1, 5, 0, 6];

type Cache = FnvHashMap<u64, i32>;

pub struct Solver {
    pub visited: usize,
    pub cache: Cache,
}

const CACHE_SIZE: usize = 1 << 10 + 1;

impl Solver {
    pub fn new() -> Solver {
        Solver {
            visited: 0,
            cache: Cache::with_capacity_and_hasher(CACHE_SIZE, Default::default()),
        }
    }

    pub fn solve(&mut self, p: Position) -> i32 {
        let mut min = -((WIDTH * HEIGHT - p.play_count) as i32) / 2;
        let mut max = ((WIDTH * HEIGHT + 1 - p.play_count)) as i32 / 2;

        while min < max {
            let mut mid = min + (max - min) / 2;

            if      mid <= 0 && min / 2 < mid { mid = min / 2; } // ?? (improve x2 but why?)
            else if mid >= 0 && max / 2 > mid { mid = max / 2; } // ??

            let score = self.solve_rec(p.clone(), mid, mid + 1);
            if score > mid { min = score; }
            else           { max = score; }
        }
        return min;
    }

    // the weak solver only tells if the position is a win/lose/draw
    // it's faster but less precise
    pub fn solve_weak(&mut self, p: Position) -> i32 {
        self.solve_rec(p, -1, 1)
    }

    fn solve_rec(&mut self, p: Position, mut alpha: i32, mut beta: i32) -> i32 {
        self.visited += 1;
        if p.is_draw() {
            return 0;
        }
        for x in 0..WIDTH {
            if p.is_valid_play(x) && p.is_winning_play(x) {
                return (((WIDTH * HEIGHT + 1) as i32) - (p.play_count as i32)) / 2;
            }
        }

        if let Some(max_score) = self.cache.get(&p.key()) {
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

        let mut best = alpha;
        for x in (0..(WIDTH as usize))
                    .map(|x| COLUMNS_ORDER[x])
                    .filter(|x| p.is_valid_play(*x))
        {
            // using negamax, variante of minimax where:
            // max(player1, player2) == -min(-player1, -player2)
            let score = -self.solve_rec(p.play(x), -beta, -alpha);
            if score > best {
                best = score;
                // reduce alpha-beta range if found better score
                if best > alpha {
                    alpha = best;
                }
                // impossible alpha-beta range reached (alpha is supposed to be < to beta)
                if alpha >= beta {
                    return score;
                }
            }
        }
        self.cache.insert(p.key(), best);
        return best;
    }

    pub fn reset(&mut self) {
        self.visited = 0;
        self.cache.clear();
    }
}
