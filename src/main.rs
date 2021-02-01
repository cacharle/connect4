const HEIGHT:      u64 = 6;
const WIDTH:       u64 = 7;
const FULL_HEIGHT: u64 = HEIGHT + 1;

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    CurrentPlayer,
    OtherPlayer,
}

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
#[derive(Clone)]
struct Position {
    /// stones of the current player
    player: u64,
    /// stones of the grid
    mask: u64,
    pub play_count: u64,
}

// https://github.com/PascalPons/connect4/blob/master/Position.hpp
impl Position {
    fn new() -> Position {
        Position{ player: 0, mask: 0, play_count: 0 }
    }

    fn play(&self, col_pos: u64) -> Position {
        Position {
            player:     self.player ^ self.mask,
            mask:       self.mask | self.mask + Position::bottom_mask(col_pos),
            play_count: self.play_count + 1,
        }
    }

    fn is_valid_play(&self, col_pos: u64) -> bool {
        self.mask & Position::top_mask(col_pos) == 0
    }

    fn is_winning_play(&self, col_pos: u64) -> bool {
        // the current player stones with the play
        // mask + bottom_mask = add a stone in column
        // & column_mask: only modify the current player stone
        // (ignoring other player in other columns)
        let p = self.player | ((self.mask + Position::bottom_mask(col_pos)) & Position::column_mask(col_pos));

        // shift the board in different direction,
        // if the shifed boards have at least 1 stone in common
        // it means that the original has 4 stone aligned.
        // possible optimization with tmp variable to make 2 shifts instead of 4

        // vertical
        if (p >> 0) &
           (p >> 1) &
           (p >> 2) &
           (p >> 3) != 0 {
            return true;
        }
        // horizontal
        if (p >> (0 * FULL_HEIGHT)) &
           (p >> (1 * FULL_HEIGHT)) &
           (p >> (2 * FULL_HEIGHT)) &
           (p >> (3 * FULL_HEIGHT)) != 0 {
            return true;
        }
        // diagonal
        if (p >> (0 * HEIGHT)) &
           (p >> (1 * HEIGHT)) &
           (p >> (2 * HEIGHT)) &
           (p >> (3 * HEIGHT)) != 0 {
            return true;
        }
        // anti diagonal
        if (p >> (0 * (HEIGHT + 2))) &
           (p >> (1 * (HEIGHT + 2))) &
           (p >> (2 * (HEIGHT + 2))) &
           (p >> (3 * (HEIGHT + 2))) != 0 {
            return true;
        }
        false
    }

    fn is_draw(&self) -> bool {
        return self.play_count == WIDTH * HEIGHT;
    }

    fn key(&self) -> u64 {
        self.player + self.mask
    }

    fn bottom_mask(col_pos: u64) -> u64 {
        1 << (col_pos * FULL_HEIGHT)
    }

    fn top_mask(col_pos: u64) -> u64 {
        Position::bottom_mask(col_pos) << (HEIGHT - 1)
    }

    fn column_mask(col_pos: u64) -> u64 {
        0b1111111 << (col_pos * FULL_HEIGHT)
        // ((1 << HEIGHT) - 1) << (col_pos * FULL_HEIGHT)
    }

    fn at(&self, y: u64, x: u64) -> Cell {
        let pos_mask = (1 << (x * FULL_HEIGHT)) << y;
        if self.mask & pos_mask == 0 {
            return Cell::Empty;
        }
        if self.player & pos_mask != 0 {
            return Cell::CurrentPlayer;
        }
        return Cell::OtherPlayer;
    }
}

impl From<&[u64]> for Position {
    fn from(plays: &[u64]) -> Self {
        let mut position = Position::new();
        for col_pos in plays {
            position = position.play(*col_pos);
        }
        position
    }
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let it = s.chars().map(|c| c.to_digit(10).unwrap() as u64 - 1);
        if it.clone().any(|x| x >= WIDTH) {
            panic!("bad position string format \"{}\"", s);
        }
        Position::from(
            &it.collect::<Vec<u64>>()[..]
        )
    }
}

use std::fmt;

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Cell::*;
        write!(f, "play_count: {}\n", self.play_count)?;
        write!(f, "{:10}{:10}{:10}\n", "position", "mask", "player")?;
        for y in (0..FULL_HEIGHT).rev() {
            for x in 0..WIDTH {
                match self.at(y, x) {
                    Empty         => write!(f, ".")?,
                    CurrentPlayer => write!(f, "x")?,
                    OtherPlayer   => write!(f, "o")?,
                }
            }
            write!(f, "  ")?;
            for x in 0..WIDTH {
                match self.at(y, x) {
                    Empty                       => write!(f, ".")?,
                    CurrentPlayer | OtherPlayer => write!(f, "#")?,
                }
            }
            write!(f, "  ")?;
            for x in 0..WIDTH {
                match self.at(y, x) {
                    Empty | OtherPlayer => write!(f, ".")?,
                    CurrentPlayer       => write!(f, "#")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Position::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                assert_eq!(p.at(y, x), Cell::Empty);
            }
        }
    }

    #[test]
    fn test_play() {
        let mut p = Position::new();
        p = p.play(0);
        assert_eq!(p.at(0, 0), Cell::OtherPlayer,   "\n{:?}", p);
        p = p.play(0);
        assert_eq!(p.at(0, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(1, 0), Cell::OtherPlayer,   "\n{:?}", p);
        p = p.play(0);
        assert_eq!(p.at(0, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(1, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(2, 0), Cell::OtherPlayer,   "\n{:?}", p);
        p = p.play(0);
        assert_eq!(p.at(0, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(1, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(2, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(3, 0), Cell::OtherPlayer,   "\n{:?}", p);

        p = p.play(1);
        assert_eq!(p.at(0, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(1, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(2, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(3, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(0, 1), Cell::OtherPlayer,   "\n{:?}", p);

        p = p.play(WIDTH - 1);
        assert_eq!(p.at(0, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(1, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(2, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(3, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(0, 1), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(0, WIDTH - 1), Cell::OtherPlayer, "\n{:?}", p);
    }

    #[test]
    fn test_is_valid_play() {
        let mut p = Position::new();
        for c in 0..WIDTH {
            for _ in 0..HEIGHT {
                assert!(p.is_valid_play(c));
                p = p.play(c);
            }
            assert!(!p.is_valid_play(c));
        }
    }

    fn assert_not_winning_play(p: Position, col_pos: u64) -> Position {
        assert!(!p.is_winning_play(col_pos), "\n{:?}", p);
        p.play(col_pos)
    }

    #[test]
    fn test_is_winning_play() {
        let mut p = Position::new();
        p = assert_not_winning_play(p, 0);
        p = assert_not_winning_play(p, WIDTH - 1);
        p = assert_not_winning_play(p, 1);
        p = assert_not_winning_play(p, WIDTH - 1);
        p = assert_not_winning_play(p, 2);
        p = assert_not_winning_play(p, WIDTH - 1);
        assert!(p.is_winning_play(3), "\n{:?}", p);  // horizontal
        p = p.play(4);
        assert!(p.is_winning_play(WIDTH - 1), "\n{:?}", p);  // vertical

        p = Position::new();
        p = assert_not_winning_play(p, 3); // w
        p = assert_not_winning_play(p, 2);
        p = assert_not_winning_play(p, 2); // w
        p = assert_not_winning_play(p, 1);
        p = assert_not_winning_play(p, 0); // w
        p = assert_not_winning_play(p, 1);
        p = assert_not_winning_play(p, 1); // w
        p = assert_not_winning_play(p, 0);
        p = assert_not_winning_play(p, 5); // w
        p = assert_not_winning_play(p, 0);
        assert!(p.is_winning_play(0), "\n{:?}", p);  // diagonal

        p = Position::new();
        p = assert_not_winning_play(p, 0); // w
        p = assert_not_winning_play(p, 1);
        p = assert_not_winning_play(p, 1); // w
        p = assert_not_winning_play(p, 2);
        p = assert_not_winning_play(p, 3); // w
        p = assert_not_winning_play(p, 2);
        p = assert_not_winning_play(p, 2); // w
        p = assert_not_winning_play(p, 3);
        p = assert_not_winning_play(p, 5); // w
        p = assert_not_winning_play(p, 3);
        assert!(p.is_winning_play(3), "\n{:?}", p);  // anti diagonal
    }

    #[test]
    fn test_is_draw() {
    }

    #[test]
    fn test_from_slice() {
        let p = Position::from(&[0, 1, 2][..]);
        assert_eq!(p.at(0, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(0, 1), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(0, 2), Cell::OtherPlayer,   "\n{:?}", p);

        let p = Position::from(&[0, 0, 0][..]);
        assert_eq!(p.at(0, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(1, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(2, 0), Cell::OtherPlayer,   "\n{:?}", p);
    }

    #[test]
    fn test_from_str() {
        let p = Position::from("012");
        assert_eq!(p.at(0, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(0, 1), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(0, 2), Cell::OtherPlayer,   "\n{:?}", p);

        let p = Position::from("000");
        assert_eq!(p.at(0, 0), Cell::OtherPlayer,   "\n{:?}", p);
        assert_eq!(p.at(1, 0), Cell::CurrentPlayer, "\n{:?}", p);
        assert_eq!(p.at(2, 0), Cell::OtherPlayer,   "\n{:?}", p);

        assert!(std::panic::catch_unwind(|| Position::from("a")).is_err());
        assert!(std::panic::catch_unwind(|| Position::from("7")).is_err());
        assert!(std::panic::catch_unwind(|| Position::from("00 0")).is_err());
    }
}

fn solve(p: Position) -> i32 {
    if p.is_draw() {
        return 0;
    }
    for x in 0..WIDTH {
        if p.is_valid_play(x) && p.is_winning_play(x) {
            return (((WIDTH * HEIGHT + 1) as i32) - (p.play_count as i32)) / 2;
        }
    }
    let mut best = -((WIDTH * HEIGHT) as i32);
    for x in 0..WIDTH {
        if !p.is_valid_play(x) {
            continue
        }
        let score = -solve(p.play(x));
        if score > best {
            best = score;
        }
    }
    return best;
}

fn main() {
    let mut p = Position::from("7422341735647741166133573473242566");
    // p = p.play(2);
    // p = p.play(2);
    // p = p.play(1);
    // p = p.play(5);
    // println!("{:?}", p);
    println!("{}", solve(p.clone()));
}
