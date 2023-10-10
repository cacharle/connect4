use bitfield_struct::bitfield;

use crate::position::{Position, HEIGHT, WIDTH};

const COLUMNS_ORDER: [u64; 7] = [3, 2, 4, 1, 5, 0, 6];

pub struct Solver {
    pub visited: usize,
    cache: Cache,
}

const CACHE_SIZE: usize = 1_000_000;

#[bitfield(u64, default = true)]
struct CacheEntry {
    #[bits(56)]
    key: u64,
    #[bits(8)]
    value: i32,
}

struct Cache(Vec<CacheEntry>);

impl Cache {
    pub fn new() -> Cache {
        Cache(vec![Default::default(); CACHE_SIZE])
    }

    pub fn insert(&mut self, key: u64, value: i32) {
        self.0[Cache::index(key)] = CacheEntry::new().with_key(key).with_value(value);
    }

    pub fn get(&self, key: u64) -> i32 {
        let entry = &self.0[Cache::index(key)];
        return if entry.key() == key { entry.value() } else { 0 };
    }

    pub fn clear(&mut self) {
        self.0.fill(Default::default());
    }

    fn index(key: u64) -> usize {
        (key % CACHE_SIZE as u64) as usize
    }
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            visited: 0,
            cache: Cache::new(),
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

        let non_losing_play_mask = p.possible_non_losing_play_mask();
        if non_losing_play_mask == 0 {
            // not width*height [+ 1] because it's one less move
            return -(((WIDTH * HEIGHT) as i32) - (p.play_count as i32)) / 2;
        }

        if p.is_draw() {
            return 0;
        }
        // for x in 0..WIDTH {
        //     if p.is_valid_play(x) && p.is_winning_play(x) {
        //         return (((WIDTH * HEIGHT + 1) as i32) - (p.play_count as i32)) / 2;
        //     }
        // }

        let max_score = self.cache.get(p.key());
        if max_score != 0 {
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

        let mut plays: Vec<_> = COLUMNS_ORDER
            .iter()
            .filter(|&&x| p.is_valid_play(x))
            .collect();
        // use insert sort instead (is there optimal instructions for a 8 size array?)
        plays.sort_by_key(|&&x| 100 - p.play(x).opponent().score());
        let mut best = alpha;
        for &x in plays {
            let played = p.play(x);
            if Position::column_mask(x) & non_losing_play_mask == 0 {
                continue;
            }
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
