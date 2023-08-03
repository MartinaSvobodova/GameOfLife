use crate::cell::Cell;

pub struct Map {
    content: Vec<Vec<Cell>>,
}

impl Iterator for Map {
    type Item = Map;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_content: Vec<Vec<Cell>> = vec![];
        for (index_row, cell_row) in self.content.iter().enumerate() {
            let mut curr_cell_row: Vec<Cell> = vec![];
            for (index_column, cell) in cell_row.iter().enumerate(){
            }
        }
        Some(new_content);
    }
}

impl Map {
    pub fn create_from_map(field: Vec<Vec<Cell>>) -> Map{
        Map{
            content: field
        }
    }
    pub fn get_new_cell(&self, row: usize, col: usize){
        for r in (row - 1)..(row + 2){
            for c in (col - 1)..(col + 2){
                if (0 <= r) && ( r < self.content.len()) && (0 <= c) && (c < self.content.first().unwrap().len()){
                }
            }
        }
    }
}