pub mod position;
pub mod solve;

use position::Position;
use solve::solve;

fn main() {




    let mut p = Position::from("7422341735647741166133573473242566");
    // p = p.play(2);
    // p = p.play(2);
    // p = p.play(1);
    // p = p.play(5);
    // println!("{:?}", p);
    println!("{}", solve(p.clone(), -10000, 100000));
}
