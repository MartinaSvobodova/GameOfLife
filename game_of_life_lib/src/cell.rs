#[derive(Debug, Clone)]
pub enum Cell {
    Alive, Dead
}
impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Alive => "O".to_owned(),
            Cell::Dead => " ".to_owned(),
        }
    }
}