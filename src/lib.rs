pub mod board;
pub mod movement;
pub mod pieces;
use std::io;
pub fn input() -> (usize, usize) {
    let mut f_row = String::new();
    let mut f_col = String::new();
    io::stdin().read_line(&mut f_row).unwrap();
    let f_row: usize = f_row.trim().parse().unwrap();
    io::stdin().read_line(&mut f_col).unwrap();
    let f_col: usize = f_col.trim().parse().unwrap();

    return (f_row, f_col);
}
