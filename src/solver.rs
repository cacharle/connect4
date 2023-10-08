use fnv::FnvHashMap; // better hashing function performance on small key (e.g u64)

use crate::position::{Position, HEIGHT, WIDTH};

const COLUMNS_ORDER: [u64; 7] = [3, 2, 4, 1, 5, 0, 6];

type Cache = FnvHashMap<u64, i32>;

pub struct Solver {
    pub visited: usize,
    cache: Cache,
}

const CACHE_SIZE: usize = 64 * 1000;
// struct Cache(Vec<(u64, i32)>);
//
// impl Cache {
//     pub fn new() -> Cache {
//         Cache(vec![(0, 0); CACHE_SIZE])
//     }
//
//     pub fn insert(&mut self, key: u64, value: i32) {
//         let i = Cache::index(key);
//         self.0[i].0 = key;
//         self.0[i].1 = value;
//     }
//
//     pub fn get(&self, key: u64) -> i32 {
//         let i = Cache::index(key);
//         return if self.0[i].0 == key {
//             self.0[i].1
//         } else {
//             0
//         }
//     }
//
//     pub fn clear(&mut self) {
//         self.0.fill((0, 0));
//     }
//
//     fn index(key: u64) -> usize {
//         (key % CACHE_SIZE as u64) as usize
//     }
// }

impl Solver {
    pub fn new() -> Solver {
        Solver {
            visited: 0,
            cache: Cache::with_capacity_and_hasher(CACHE_SIZE, Default::default()),
            // cache: Cache::new(),
        }
    }

    pub fn solve(&mut self, p: Position) -> i32 {
        let mut min = -((WIDTH * HEIGHT - p.play_count) as i32) / 2;
        let mut max = (WIDTH * HEIGHT + 1 - p.play_count) as i32 / 2;
        // Iterative deepening
        // -------------------
        // Increase the search depth step by step
        // 1. Put early results in transposition table
        // 2. Explore shallow winning paths first
        // My undestanding:
        // We don't want to get stuck searching at a very big depth if the winning
        // move is somewhere else entirely
        // Starting at a shallow depth allows to more broadly explore the possibilities and not get
        // tunnel-vision.
        // We take advantage that alpha,beta are scores which are the *number of moves* before the
        // end. Meaning that reducing the alpha,beta range will reduce the depth of the search.
        // Here `mid` IS the depth at which we search
        //
        // Null window search
        // ------------------
        // Setting alpha to beta-1 to prune more positions quicker.
        // My undestanding:
        // We do this to allow us to to a dichotomic search on the actual score.
        //
        // `mid` starts at 0 and then become larger or greater. Meaning we start at low depth and
        // explore more and more deep in one direction (winning/+ or losing/-).
        while min < max {
            // Compute mid according to min,max to in a dichotomic search fashion
            let mut mid = min + (max - min) / 2;
            if mid <= 0 && min / 2 < mid {
                mid = min / 2;
            } else if mid >= 0 && max / 2 > mid {
                mid = max / 2;
            }
            // Check if actual score is greater or lower than mid
            let shallow_score = self.solve_rec(p.clone(), mid, mid + 1);
            // Reduce the min,max bounds according to shallow score
            if shallow_score > mid {
                min = shallow_score;
            } else {
                max = shallow_score;
            }
        }
        return min;
    }

    // The weak solver only tells if the position is a win/lose/draw
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

        // let max_score = self.cache.get(p.key());
        // if max_score != 0 {
        if let Some(&max_score) = self.cache.get(&p.key()) {
            // can't return max_score directly
            // because the alpha-beta context in the cache may be
            // different than the current alpha-beta
            if beta > max_score {
                beta = max_score;
                if alpha >= beta {
                    return beta;
                }
            }
        }

        let non_losing_play_mask = p.possible_non_losing_play_mask();
        let mut best = alpha;
        for &x in COLUMNS_ORDER.iter().filter(|&&x| p.is_valid_play(x)) {
            let played = p.play(x);
            // if Position::column_mask(x) & non_losing_play_mask == 0 {
            //     continue;
            // }
            // using negamax, variante of minimax where:
            // max(player1, player2) == -min(-player1, -player2)
            let score = -self.solve_rec(played, -beta, -alpha);
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
