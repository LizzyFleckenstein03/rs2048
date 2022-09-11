pub mod display;
pub mod game;

use game::{Board, Dir::*, Pos};

fn main() {
    let mut rng = rand::thread_rng();
    let getch = getch::Getch::new();
    let board = Board::new(Pos::new(4, 4));

    board.spawn(&mut rng);
    clearscreen::clear().unwrap();
    print!("{board}");

    while let Ok(ch) = getch.getch() {
        if !board.step(match ch {
            b'w' => Up,
            b'a' => Left,
            b's' => Down,
            b'd' => Right,
            b'q' => break,
            _ => continue,
        }) {
            continue;
        }

        board.spawn(&mut rng);
        clearscreen::clear().unwrap();
        print!("{board}");
    }
}
