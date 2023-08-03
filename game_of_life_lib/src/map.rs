use std::error::Error;

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
}
