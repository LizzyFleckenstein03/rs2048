use super::game::{Board, Pos};
use ansi_term::Color;
use std::fmt;

enum Mode {
    Roof_,
    Data_,
    Floor,
    Empty,
    Base_,
}

const FIELD_HEIGHT: usize = 3;
const FIELD_WIDTH: usize = 8;

fn write_line(f: &mut fmt::Formatter, vec: &[u32], mode: Mode) -> fmt::Result {
    let mut vec = vec;
    let len = vec.len();

    write!(
        f,
        "{}",
        match mode {
            Mode::Roof_ => "┏",
            Mode::Data_ => "┃",
            Mode::Floor => "┠",
            Mode::Empty => "┃",
            Mode::Base_ => "┗",
        }
    )?;

    for i in 0..len {
        let &n = vec.last().unwrap();
        vec = &vec[0..vec.len() - 1];

        match mode {
            Mode::Data_ | Mode::Empty => write!(f, "\x1b[0m ")?,
            _ => {}
        }

        let color = match mode {
            Mode::Data_ | Mode::Empty if n != 0 => {
                let (r, g, b) = hsl::HSL {
                    h: (n * 360 / 12) as f64,
                    s: 1.0,
                    l: 0.5,
                }
                .to_rgb();

                Color::Black.on(Color::RGB(r, g, b))
            }
            _ => Color::White.on(Color::Black),
        };

        if let Mode::Data_ = mode {
            if n == 0 {
                write!(f, "{}", " ".repeat(FIELD_WIDTH - 2))?;
            } else {
                write!(
                    f,
                    "{}",
                    color.paint(format!("{:^w$}", 1 << n, w = FIELD_WIDTH - 2))
                )?;
            }
        } else {
            write!(
                f,
                "{}",
                match mode {
                    Mode::Roof_ | Mode::Base_ => "━".repeat(FIELD_WIDTH),
                    Mode::Floor => "─".repeat(FIELD_WIDTH),
                    Mode::Empty => color.paint(" ".repeat(FIELD_WIDTH - 2)).to_string(),
                    Mode::Data_ => panic!("unreachable"),
                }
            )?;
        }

        match mode {
            Mode::Data_ | Mode::Empty => write!(f, " ")?,
            _ => {}
        }

        if i != len - 1 {
            write!(
                f,
                "{}",
                match mode {
                    Mode::Roof_ => "┯",
                    Mode::Data_ => "│",
                    Mode::Floor => "┼",
                    Mode::Empty => "│",
                    Mode::Base_ => "┷",
                }
            )?;
        }
    }

    write!(
        f,
        "{}",
        match mode {
            Mode::Roof_ => "┓",
            Mode::Data_ => "┃",
            Mode::Floor => "┨",
            Mode::Empty => "┃",
            Mode::Base_ => "┛",
        }
    )?;

    writeln!(f)?;

    Ok(())
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dummy = vec![0; self.size.x as usize];

        write_line(f, &dummy, Mode::Roof_)?;

        for y in 0..self.size.y {
            let vec = (0..self.size.x)
                .rev()
                .map(|x| self.get(Pos::new(x, y)).value())
                .collect::<Vec<u32>>();

            for i in 0..FIELD_HEIGHT {
                write_line(
                    f,
                    &vec,
                    if i == FIELD_HEIGHT / 2 {
                        Mode::Data_
                    } else {
                        Mode::Empty
                    },
                )?;
            }

            if y != self.size.y - 1 {
                write_line(f, &dummy, Mode::Floor)?;
            }
        }

        write_line(f, &dummy, Mode::Base_)?;

        Ok(())
    }
}
