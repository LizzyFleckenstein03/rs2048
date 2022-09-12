use super::game::{Board, Pos};
use crossterm::{cursor, queue, style::*};

enum Mode {
    Roof_,
    Data_,
    Floor,
    Empty,
    Base_,
}

const FIELD_HEIGHT: usize = 3;
const FIELD_WIDTH: usize = 8;

fn write_line(stdout: &mut std::io::Stdout, vec: &[u32], mode: Mode) -> crossterm::Result<()> {
    let mut vec = vec;
    let len = vec.len();

    queue!(
        stdout,
        Print(match mode {
            Mode::Roof_ => "┏",
            Mode::Data_ => "┃",
            Mode::Floor => "┠",
            Mode::Empty => "┃",
            Mode::Base_ => "┗",
        })
    )?;

    for i in 0..len {
        let &n = vec.last().unwrap();
        vec = &vec[0..vec.len() - 1];

        match mode {
            Mode::Data_ | Mode::Empty => queue!(stdout, Print(" "))?,
            _ => {}
        }

        match mode {
            Mode::Data_ | Mode::Empty if n != 0 => {
                let (r, g, b) = hsl::HSL {
                    h: (n * 360 / 12) as f64,
                    s: 1.0,
                    l: 0.5,
                }
                .to_rgb();

                queue!(
                    stdout,
                    SetColors(Colors::new(Color::Black, Color::Rgb { r, g, b }))
                )?;
            }
            _ => {}
        };

        if let Mode::Data_ = mode {
            if n == 0 {
                queue!(stdout, Print(" ".repeat(FIELD_WIDTH - 2)))?;
            } else {
                queue!(
                    stdout,
                    Print(format!("{:^w$}", 1 << n, w = FIELD_WIDTH - 2))
                )?;
            }
        } else {
            queue!(
                stdout,
                Print(match mode {
                    Mode::Roof_ | Mode::Base_ => "━".repeat(FIELD_WIDTH),
                    Mode::Floor => "─".repeat(FIELD_WIDTH),
                    Mode::Empty => " ".repeat(FIELD_WIDTH - 2),
                    Mode::Data_ => panic!("unreachable"),
                })
            )?;
        }

        match mode {
            Mode::Data_ | Mode::Empty => {
                queue!(stdout, Print(" "), SetAttribute(Attribute::Reset))?
            }
            _ => {}
        }

        if i != len - 1 {
            queue!(
                stdout,
                Print(match mode {
                    Mode::Roof_ => "┯",
                    Mode::Data_ => "│",
                    Mode::Floor => "┼",
                    Mode::Empty => "│",
                    Mode::Base_ => "┷",
                })
            )?;
        }
    }

    queue!(
        stdout,
        Print(match mode {
            Mode::Roof_ => "┓",
            Mode::Data_ => "┃",
            Mode::Floor => "┨",
            Mode::Empty => "┃",
            Mode::Base_ => "┛",
        }),
        cursor::MoveToNextLine(1),
    )?;

    Ok(())
}

pub fn display_board(stdout: &mut std::io::Stdout, board: &Board) -> crossterm::Result<()> {
    let dummy = vec![0; board.size.x as usize];

    write_line(stdout, &dummy, Mode::Roof_)?;

    for y in 0..board.size.y {
        let vec = (0..board.size.x)
            .rev()
            .map(|x| board.get(Pos::new(x, y)).value())
            .collect::<Vec<u32>>();

        for i in 0..FIELD_HEIGHT {
            write_line(
                stdout,
                &vec,
                if i == FIELD_HEIGHT / 2 {
                    Mode::Data_
                } else {
                    Mode::Empty
                },
            )?;
        }

        if y != board.size.y - 1 {
            write_line(stdout, &dummy, Mode::Floor)?;
        }
    }

    write_line(stdout, &dummy, Mode::Base_)?;

    Ok(())
}
