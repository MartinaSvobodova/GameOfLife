use std::ops::Not;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Alive, Dead
}
impl From<bool> for Cell {
    fn from(value: bool) -> Self {
        match value {
            true => Cell::Alive,
            false => Cell::Dead,
        }
    }
}
impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Alive => "O".to_owned(),
            Cell::Dead => " ".to_owned(),
        }
    }
}

impl Not for Cell {
    type Output = Cell;

    fn not(self) -> Self::Output {
        match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}