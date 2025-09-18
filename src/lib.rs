pub mod board;
pub mod movement;
pub mod pieces;
use std::io;
pub fn input() -> (usize, usize) {
    // This code takes input and converts to index.
    let mut x = String::new();
    let mut y = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x: char = x.trim().parse().unwrap();
    let x: u32 = x.to_digit(18).unwrap() - 10;
    io::stdin().read_line(&mut y).unwrap();
    let y: usize = y.trim().parse().unwrap();
    let y: usize = 8 - y;
    return (x as usize, y);
}
pub fn translate(m: &str) -> (usize, usize) {
    let m: Vec<_> = m.chars().collect();
    let x = m[0];
    let y = m[1] as usize;
    let x: u32 = x.to_digit(18).unwrap() - 10;
    let y: usize = 8 - y;
    return (x as usize, y);
}

// [y][x]
