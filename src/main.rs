const HEIGHT: u64 = 6;
const WIDTH:  u64 = 7;

type Bits = u64;

/**
 * bit order:
 *
 * .  .  .  .  .  .  .
 * 5 12 19 26 33 40 47
 * 4 11 18 25 32 39 46
 * 3 10 17 24 31 38 45
 * 2  9 16 23 30 37 44
 * 1  8 15 22 29 36 43
 * 0  7 14 21 28 35 42
 *
 * an extra row is added for the key represention
 *
 *
 *
 */
struct Position {
    /// stones of the current player
    player: u64,
    /// stones of the grid
    mask:   u64,
}

impl Position {
    fn new() -> Position {
        Position{ player: 0, mask: 0 }
    }

    fn from_position(col_moves: &[u64]) -> Position {
        let mut position = Position::new();
        for col_pos in col_moves {
            position.play(*col_pos);
        }
        position
    }

    fn play(&mut self, col_pos: u64) {
        self.player ^= self.mask;
        self.mask |= self.mask + (1 << (col_pos * (HEIGHT + 1)));
    }

    fn key(&self) -> u64 {
        self.player + self.mask
    }
}

use std::fmt;

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mask: {:064b}\n", self.mask)?;
        for i in 0..WIDTH {
            write!(f, "\t{:07b}\n", (self.mask & (0b1111111 << (i * WIDTH))) >> (i * WIDTH));
        }

        write!(f, "player: {:064b}\n", self.player)?;
        for i in 0..WIDTH {
            write!(f, "\t{:07b}\n", (self.player & (0b1111111 << (i * WIDTH))) >> (i * WIDTH));
        }

        write!(f, "key: {:064b}\n", self.key())?;
        for i in 0..WIDTH {
            write!(f, "\t{:07b}\n", (self.key() & (0b1111111 << (i * WIDTH))) >> (i * WIDTH));
        }
        Ok(())
    }
}

fn main() {
    let mut p = Position::new();
    // println!("{:?}", p);
    p.play(2);
    p.play(2);
    p.play(1);
    p.play(5);
    p.play(5);
    println!("{:?}", p);
    // p.play(4);
    // println!("------------------------------");
    // println!("{:?}", p);
}
