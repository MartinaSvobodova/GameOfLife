use std::{io, error::Error, fmt};
use game_of_life_lib::{map::Map, cell::Cell};

fn main() -> Result<(), Box<dyn Error>> {
    let test = vec![vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
                                    vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
                                    vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
                                    vec![Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead],
                                    vec![Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead]];
    let new_map = Map::create_from_map(test);
    // println!("Please enter the width, height and the percentage chance of the starting cells being alive in the format:num num num");
    // let mut user_input = String::new();
    // io::stdin().read_line(&mut user_input)?;
    // user_input = user_input.trim_end().to_string();
    // let string_nums = user_input.split(" ").collect::<Vec<&str>>();
    // if string_nums.len() != 3{
    //     return Err(Box::new(fmt::Error));
    // }
    // let mut new_map: Map = Map::create_from_size(string_nums[0].parse()?, string_nums[1].parse()?, string_nums[2].parse()?).unwrap();
    // println!("Here is your board");
    // print!("{}", new_map.to_string());
    // println!("Here is 1 iteration: ");
    // print!("{}", new_map.next().unwrap().to_string());
    // println!("Please enter the file name: ");
    // let mut user_in = String::new();
    // io::stdin().read_line(&mut user_in)?;
    // let mut tester = Map::create_from_file(user_in.trim_end().to_string())?;
    // print!("{}", tester.to_string());
    // println!("{}", tester.next().unwrap().to_string());
    new_map.write_map_into_file("testing.txt".to_string())?;
    Ok(())
}
