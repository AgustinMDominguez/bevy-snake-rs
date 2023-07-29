
pub fn min<T: PartialOrd>(a: T, b: T) -> T { if a <= b { a } else { b } }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction { Left, Right, Up, Down }

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}
