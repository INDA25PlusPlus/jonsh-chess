pub mod board;
pub mod movement;
pub mod pieces;
use std::io;
pub fn input() -> (usize, usize) {
    let mut x = String::new(); //create x variable
    let mut y = String::new(); //create y variable
    io::stdin().read_line(&mut x).unwrap(); //read input from terminal
    let x: char = x.trim().parse().unwrap(); //convert input to char i.e: input is "a\n" or "a\r\n" and a is output
    let x: u32 = x.to_digit(18).unwrap() - 10; //convert char to int
    io::stdin().read_line(&mut y).unwrap(); //read input from terminal
    let y: usize = y.trim().parse().unwrap(); //convert input to int
    let y: usize = y - 1; // -1 since index starts at 0
    return (x as usize, y); //return both values as a tuple
}
