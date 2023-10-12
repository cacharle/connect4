use bitfield_struct::bitfield;

use crate::position::{Position, HEIGHT, WIDTH};

const COLUMNS_ORDER: [u64; 7] = [3, 2, 4, 1, 5, 0, 6];

pub struct Solver {
    pub visited: usize,
    cache: Cache,
}

const CACHE_SIZE: usize = 9_500_000 / 8; // L2 cache is 9.5MB

#[bitfield(u64, default = true)]
struct CacheEntry {
    #[bits(56)]
    key: u64, // a position only needs 56bits to be represented
    #[bits(8)]
    value: i32,
}

use std::sync::RwLock;

struct Cache(RwLock<Vec<CacheEntry>>);

impl Cache {
    pub fn new() -> Cache {
        Cache(RwLock::new(vec![Default::default(); CACHE_SIZE]))
    }

    pub fn insert(&mut self, key: u64, value: i32) {
        self.0.write().unwrap()[Cache::index(key)] =
            CacheEntry::new().with_key(key).with_value(value);
    }

    pub fn get(&self, key: u64) -> i32 {
        let entry = &self.0.read().unwrap()[Cache::index(key)];
        return if entry.key() == key { entry.value() } else { 0 };
    }

    pub fn clear(&mut self) {
        self.0.write().unwrap().fill(Default::default());
    }

    fn index(key: u64) -> usize {
        (key % CACHE_SIZE as u64) as usize
    }
}

struct PlaySorter {
    plays: [(u64, u64); 8],
    size: usize,
}

impl PlaySorter {
    fn new() -> PlaySorter {
        PlaySorter {
            plays: Default::default(),
            size: 0,
        }
    }

    fn insert(&mut self, play: u64, score: u64) {
        assert!(self.size < self.plays.len());
        let mut index = 0;
        while self.plays[index].1 < score && index < self.size {
            index += 1;
        }
        self.plays[index..].rotate_right(1);
        self.plays[index] = (play, score);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<u64> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        Some(self.plays[self.size].0)
    }
}

use std::sync::{mpsc, Arc};

impl Solver {
    pub fn new() -> Solver {
        Solver {
            visited: 0,
            cache: Cache::new(),
        }
    }

    pub fn best_play(&mut self, p: Position) -> u64 {
        let (tx, rx) = mpsc::channel();
        // let self_rc = Arc::new(self);
        for &c in COLUMNS_ORDER.iter().rev() {
            let played = p.play(c).opponent();
            let tx = tx.clone();
            // TODO: pass the cache as an argument wrapped in Arc<RwLock<Cache>>
            // let self_rc = self_rc.clone();
            // std::thread::spawn(move || {
            let s = self.solve(played);
            tx.send((c, s)).unwrap();
            println!("col: {c}, score: {s}");
            // });
        }
        drop(tx);
        rx.iter().max_by_key(|&(_, score)| score).unwrap().0
    }

    pub fn solve(&mut self, p: Position) -> i32 {
        if p.is_winning() {
            // 1+ to add more weight compared to can_win_next
            return 1 + ((WIDTH * HEIGHT + 1 - p.play_count) / 2) as i32;
        }
        if p.can_win_next() {
            return ((WIDTH * HEIGHT + 1 - p.play_count) / 2) as i32;
        }
        // TODO: return directly if can win next move (otherwise solve_rec fails since it trims the
        // winning move out)
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
        debug_assert!(alpha < beta);
        debug_assert!(!p.can_win_next());
        self.visited += 1;

        let non_losing_play_mask = p.possible_non_losing_play_mask();
        if non_losing_play_mask == 0 {
            // not width*height [+ 1] because it's one less move
            return -(((WIDTH * HEIGHT) as i32) - (p.play_count as i32)) / 2;
        }

        if p.is_draw() {
            return 0;
        }

        // This copy paste made a huge difference, hmmm
        let min = -(((WIDTH * HEIGHT - 2 - p.play_count) / 2) as i32); // lower bound of score as opponent cannot win next move
        if alpha < min {
            alpha = min; // there is no need to keep beta above our max possible score.
            if alpha >= beta {
                return alpha;
            } // prune the exploration if the [alpha;beta] window is empty.
        }

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

        let mut sorter = PlaySorter::new();
        COLUMNS_ORDER
            .iter()
            .filter(|&&x| p.is_valid_play(x))
            .for_each(|&x| sorter.insert(x, p.play(x).opponent().score()));
        let mut best = alpha;
        while let Some(x) = sorter.pop() {
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

#[cfg(test)]
mod play_sorter_test {
    use super::*;
    #[test]
    fn test_insert_pop() {
        let mut s = PlaySorter::new();
        s.insert(1, 1);
        s.insert(2, 30);
        s.insert(3, 15);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }
}
