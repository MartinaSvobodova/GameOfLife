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
                curr_cell_row.push(self.get_new_cell(index_row, index_column, cell.clone()))
            }
            new_content.push(curr_cell_row);
        }
        Some(Map::create_from_map(new_content))
    }
}

impl Map {
    pub fn create_from_map(field: Vec<Vec<Cell>>) -> Map{
        Map{
            content: field
        }
    }
    pub fn get_new_cell(&self, row: usize, col: usize, curr_cell: Cell) -> Cell{
        let mut n_of_alive = 0;
        for r in (row - 1)..(row + 2){
            for c in (col - 1)..(col + 2){
                if (r == row) && (c == col) {
                    continue;
                }
                if (0 <= r) && ( r < self.content.len()) && (0 <= c) && (c < self.content.first().unwrap().len()){
                    if self.content.get(r).unwrap().get(c).unwrap() == &Cell::Alive {
                        n_of_alive = n_of_alive + 1;
                    }
                }
            }
        }
        match curr_cell {
            Cell::Alive => if (n_of_alive < 2) || (n_of_alive > 3) {
                !curr_cell
            }else{
                curr_cell
            },
            Cell::Dead => if n_of_alive == 3 {
                !curr_cell
            }else{
                curr_cell
            },
        }
    }
}