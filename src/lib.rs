pub mod board;
pub mod movement;
pub mod pieces;
use std::io;
pub fn input() -> (usize, usize) {
    let mut x = String::new();
    let mut y = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x: usize = x.trim().parse().unwrap();
    io::stdin().read_line(&mut y).unwrap();
    let y: usize = y.trim().parse().unwrap();

    return (x, y);
}
