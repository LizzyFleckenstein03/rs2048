pub use glam::i32::IVec2 as Pos;
use rand::seq::IteratorRandom;
use std::cell::RefCell;

trait Swap {
    fn swap(self) -> Self;
}

impl Swap for Pos {
    fn swap(self) -> Self {
        Self::new(self.y, self.x)
    }
}

pub struct Field(RefCell<u32>);

enum MergeResult {
    Merged(u32),
    Replaced,
    Blocked,
    Empty,
}

impl Field {
    fn new() -> Self {
        Self(RefCell::new(0))
    }

    pub fn value(&self) -> u32 {
        *self.0.borrow()
    }

    fn merge(&self, other: &Self) -> MergeResult {
        let mut s = self.0.borrow_mut();
        let mut o = other.0.borrow_mut();

        if *o == 0 {
            return MergeResult::Empty;
        }

        if *s == 0 {
            *s = *o;
            *o = 0;

            return MergeResult::Replaced;
        }

        if *s == *o {
            *s += 1;
            *o = 0;

            return MergeResult::Merged(1 << *s);
        }

        MergeResult::Blocked
    }
}

pub struct Board {
    pub size: Pos,
    fields: Vec<Vec<Field>>,
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Board {
    pub fn new(size: Pos) -> Self {
        Self {
            size,
            fields: (0..size.x)
                .map(|_| (0..size.y).map(|_| Field::new()).collect())
                .collect(),
        }
    }

    pub fn step(&self, dir: Dir) -> Option<u32> {
        let dir = match dir {
            Dir::Up => -Pos::Y,
            Dir::Down => Pos::Y,
            Dir::Left => -Pos::X,
            Dir::Right => Pos::X,
        };

        let step_row = dir.abs().swap();
        let step_col = -dir;

        let len_row = (step_row.abs() * self.size).max_element();
        let len_col = (step_col.abs() * self.size).max_element();

        let start = (dir + Pos::ONE) / 2 * (self.size - Pos::ONE);

        let mut score = None;

        for row in 0..len_row {
            let start_row = start + row * step_row;

            for col1 in 0..len_col - 1 {
                let field1 = self.get(start_row + col1 * step_col);

                for col2 in col1 + 1..len_col {
                    let field2 = self.get(start_row + col2 * step_col);

                    match field1.merge(field2) {
                        MergeResult::Merged(sc) => {
                            score = Some(score.unwrap_or(0) + sc);
                            break;
                        }
                        MergeResult::Replaced => score = Some(score.unwrap_or(0)),
                        MergeResult::Blocked => break,
                        MergeResult::Empty => continue,
                    }
                }
            }
        }

        score
    }

    pub fn get(&self, pos: Pos) -> &Field {
        self.fields
            .get(pos.x as usize)
            .unwrap()
            .get(pos.y as usize)
            .unwrap()
    }

    pub fn spawn<R>(&self, rng: &mut R)
    where
        R: rand::Rng + ?core::marker::Sized,
    {
        if let Some(field) = self
            .fields
            .iter()
            .flat_map(|v| v.iter())
            .filter(|f| f.value() == 0)
            .choose(rng)
        {
            *field.0.borrow_mut() = 1;
        }
    }
}
