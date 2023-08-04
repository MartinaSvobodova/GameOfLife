use std::{ops::Not, io::{Error, self}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Alive, Dead
}

impl Cell {
    fn to_string_custom(&self, alive: Option<String>, dead: Option<String>) -> Result<String, Error> {
        if alive.is_none() && dead.is_none() {
            return Ok(self.to_string());
        }else if alive.is_none() || dead.is_none() {
            return Err(io::ErrorKind::InvalidData.into());
        }else {
            match self {
                Cell::Alive => Ok(alive.unwrap().to_owned()),
                Cell::Dead => Ok(dead.unwrap().to_owned()),
            }
        }
    }
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
            Cell::Dead => "X".to_owned(),
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