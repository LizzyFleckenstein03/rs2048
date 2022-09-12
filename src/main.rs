pub mod display;
pub mod game;

use crossterm::{cursor, event, execute, queue, style, terminal};
use game::{Board, Dir::*, Pos};
use std::io::Write;

fn main() {
    let mut rng = rand::thread_rng();

    let mut stdout = std::io::stdout();
    queue!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();

    terminal::enable_raw_mode().unwrap();

    let board = Board::new(Pos::new(4, 4));
    board.spawn(&mut rng);
    board.spawn(&mut rng);

    let mut score = 0;

    loop {
        queue!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
            style::SetAttribute(style::Attribute::Bold),
            style::Print("Score: ".to_string()),
            style::SetAttribute(style::Attribute::Reset),
            style::Print(score.to_string()),
            cursor::MoveToNextLine(1),
        )
        .unwrap();
        display::display_board(&mut stdout, &board).unwrap();
        stdout.flush().unwrap();

        if let Ok(evt) = event::read() {
            match evt {
                event::Event::Key(event::KeyEvent { code, .. }) => match code {
                    event::KeyCode::Char(ch) => {
                        if let Some(sc) = board.step(match ch.to_ascii_lowercase() {
                            'w' => Up,
                            'a' => Left,
                            's' => Down,
                            'd' => Right,
                            'q' => break,
                            _ => continue,
                        }) {
                            score += sc;
                            board.spawn(&mut rng);
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            break;
        }
    }

    terminal::disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
}
