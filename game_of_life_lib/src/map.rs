use std::{fs::{self, File}, io::{self, Write}, error::Error};

use rand::Rng;

use crate::cell::Cell;
#[derive(Debug)]
pub struct Map {
    content: Vec<Vec<Cell>>,
}

impl Iterator for Map {
    type Item = Map;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_content: Vec<Vec<Cell>> = vec![];
        for (index_row, cell_row) in self.content.iter().enumerate() {
            let mut curr_cell_row: Vec<Cell> = vec![];
            for (index_column, cell) in cell_row.iter().enumerate() {
                curr_cell_row.push(self.get_new_cell(index_row, index_column, cell.clone()))
            }
            new_content.push(curr_cell_row);
        }
        Some(Map::create_from_map(new_content))
    }
}

impl ToString for Map {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.content {
            for curr_cell in row {
                result.push_str(&curr_cell.to_string());
            }
            result.push('\n');
        }
        result
    }
}

impl Map {
    pub fn create_from_map(field: Vec<Vec<Cell>>) -> Map {
        Map { content: field }
    }

    pub fn create_from_size(width: usize, height: usize, percentage: u8) -> Result<Map, String> {
        if percentage > 100 {
            return Err("More than 100% entered".to_string());
        }
        let mut random = rand::thread_rng();
        Ok(Map::create_from_map(
            (0..height)
                .map(|_| {
                    (0..width).map(|_| {
                        Cell::from(
                            random.gen_range(0..100) < percentage,
                        )
                            }).collect()
                })
                .collect(),
        ))
    }

    pub fn create_from_file(path: String) -> Result<Map, io::Error> {
        let content = fs::read_to_string(path)?;
        let mut lines = content.lines();
        let first_line = lines.next().unwrap().split(" ").collect::<Vec<&str>>();
        if first_line.len() != 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, ""));
        }
        let live_cell = first_line[0].chars().nth(0).unwrap();
        let width: usize = first_line[2].parse().unwrap();
        let height = lines.clone().count();
        let mut final_field: Vec<Vec<Cell>> = vec![vec![]];
        for _r in 0..height{
            let curr_line = lines.next().unwrap().chars().collect::<Vec<char>>();
            if curr_line.len() != width {
                return Err(io::Error::new(io::ErrorKind::InvalidData, ""));
            }
            let mut curr_field_line: Vec<Cell> = vec![];
            for c in 0..width{
                if curr_line[c].eq_ignore_ascii_case(&live_cell){
                    curr_field_line.push(Cell::Alive);
                }else {
                    curr_field_line.push(Cell::Dead);
                }
            }
            final_field.push(curr_field_line);
        }
        Ok(Map::create_from_map(final_field))
    }

    pub fn get_new_cell(&self, row: usize, col: usize, curr_cell: Cell) -> Cell {
        let mut n_of_alive = 0;
        for r in (row.checked_sub(1).unwrap_or(0))..(row + 2) {
            for c in (col.checked_sub(1).unwrap_or(0))..(col + 2) {
                if (r == row) && (c == col) {
                    continue;
                }
                if (r < self.content.len()) && (c < self.content.first().unwrap().len()) {
                    if self.content[r][c] == Cell::Alive {
                        n_of_alive = n_of_alive + 1;
                    }
                }
            }
        }
        match curr_cell {
            Cell::Alive => {
                if (n_of_alive < 2) || (n_of_alive > 3) {
                    !curr_cell
                } else {
                    curr_cell
                }
            }
            Cell::Dead => {
                if n_of_alive == 3 {
                    !curr_cell
                } else {
                    curr_cell
                }
            }
        }
    }

    pub fn write_map_into_file(&self, path: String) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(path).unwrap();
        writeln!(&mut file, "{} {} {}", Cell::Alive.to_string(), Cell::Dead.to_string(), self.content[0].len())?;
        for row in self.content.iter() {
            let mut curr_row: String = String::new();
            for cell in row {
                curr_row.push_str(&cell.to_string());
                curr_row.push(' ');
            }
            curr_row = curr_row.trim_end().to_string();
            writeln!(&mut file, "{}", curr_row)?;
        }
        Ok(())
    }
}
