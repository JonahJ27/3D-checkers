use std::{fmt, io::empty};

use crate::board;

#[derive(PartialEq, Clone, Copy)]
pub enum Tile {
    Empty,
    Black,
    Red,
    BlackRoyals,
    RedRoyals
}

pub struct BoardState {
    pub data: Vec<Vec<Tile>>
}

impl BoardState{
    // Checks if a move is valid. If not is will return Err result explaining why the move is invalid.
    // Otherwise it makes the move
    pub fn try_to_move(&mut self, mut position_list: Vec<String>, red_turn: bool) -> Result<String, String> {
        let mut taken_piece_posns = Vec::new();
        let first_pos = position_list.pop().ok_or("Not enough positions entered for a move.")?;

        let mut curr_col = pos_to_col(first_pos.clone())?;
        let mut curr_row = pos_to_row(first_pos.clone())?;
        println!("row: {}, col: {}", curr_row, curr_col);
        // need checking for length one lists
        while position_list.len() > 0 {
            let next_pos:String = position_list.pop().ok_or(
                "Not enough positions entered for a move.")?;
            let next_col = pos_to_col(next_pos.clone())?;
            let next_row = pos_to_row(next_pos.clone())?; 
            match self.is_valid_move(curr_row, curr_col, next_row, next_col, red_turn) {
                Ok(valid) => if valid {
                    if let Some(posn) = self.get_taken_piece(curr_row, curr_col, next_row, next_col) {
                        taken_piece_posns.push(posn);
                    }
                }
                else {
                    return Err("Invalid Move".to_owned());
                }
                Err(s) => return Err(s),
            }
            curr_row = next_row;
            curr_col = next_col;
        }
        let first_pos_type = self.data[pos_to_row(first_pos.clone())? as usize][pos_to_col(first_pos.clone())? as usize];
        self.data[pos_to_row(first_pos.clone())? as usize][pos_to_col(first_pos.clone())? as usize] = Tile::Empty;
        self.data[curr_row as usize][curr_col as usize] = first_pos_type;

        
        // add removal of taken pieces
        return Ok("Valid Move".to_owned());
    }

    // returns the type of tile that a given position is given a row and column
    // is a result that will return an error if out of bounds.
    fn get_tile_at(&self, r: u32, c: u32) -> Result<&Tile, String> {
        match self.data.get(r as usize) {
            None => return Err("Row index out of bounds".to_owned()),
            Some(row) => match row.get(c as usize) {
                None => return Err("Column index out of bounds.".to_owned()),
                Some(tile) => return Ok(tile)
            },
        }
    } 

    fn is_valid_move(&self, curr_row: u32, curr_col: u32, next_row: u32, 
                     next_col: u32, red_turn: bool) -> Result<bool, String> {
        let curr_tile = self.get_tile_at(curr_row, curr_col)?;
        let next_tile = self.get_tile_at(next_row, next_col)?;
        let mut is_royal = false;    
        if red_turn {
            match curr_tile {
                Tile::Empty => return Err("Target position".to_owned() + " is empty."),
                Tile::Black => return Err("Target position".to_owned() + " is black."),
                Tile::Red => is_royal = false,
                Tile::BlackRoyals => return Err("Target position".to_owned() + " is black royal."),
                Tile::RedRoyals => is_royal = true,
            }
        }
        else {
            match curr_tile {
                Tile::Empty => return Err("Target position".to_owned() + " is empty."),
                Tile::Black => is_royal = false,
                Tile::Red => return Err("Target position".to_owned() + " is red."),
                Tile::BlackRoyals => is_royal = true,
                Tile::RedRoyals => return Err("Target position".to_owned() + " is red royal."),
            }
        }

        if next_tile != &Tile::Empty {
            return Ok(false);
        }
        if is_royal {
            if (curr_row as i32 - next_row as i32).abs() == 2 && 
               (curr_col as i32 - next_col as i32).abs() == 2 {
                let tile_to_take = self.get_tile_at(
                    (curr_row as i32 + (curr_row as i32 - next_row as i32)) as u32,
                    (curr_col as i32 + (curr_col as i32 - next_col as i32)) as u32)?;

                return Ok(((tile_to_take == &Tile::Black || tile_to_take == &Tile::BlackRoyals) 
                    && red_turn) || ((tile_to_take == &Tile::Red || 
                    tile_to_take == &Tile::RedRoyals) && !red_turn));
            }
            else {
                return Ok((curr_col as i32 - next_col as i32).abs() == 1 && 
                    (curr_col as i32 - next_col as i32).abs() == 1);
            }
        }
        else {
            if red_turn {
                if (curr_row as i32 - next_row as i32) == -2 &&
                   (curr_col as i32 - next_col as i32).abs() == 2 {
                    let tile_to_take = self.get_tile_at(
                    (curr_row as i32 + (curr_row as i32 - next_row as i32)) as u32,
                    (curr_col as i32 + (curr_col as i32 - next_col as i32)) as u32)?;

                    return Ok(tile_to_take == &Tile::Black || tile_to_take == &Tile::BlackRoyals);
                }
                else {
                    return Ok((curr_row as i32 - next_row as i32) == -1 &&
                            (curr_col as i32 - next_col as i32).abs() == 1)
                }
            }
            else {
                if (curr_row as i32 - next_row as i32) == 2 &&
                   (curr_col as i32 - next_col as i32).abs() == 2 {
                    let tile_to_take = self.get_tile_at(
                    (curr_row as i32 + (curr_row as i32 - next_row as i32)) as u32,
                    (curr_col as i32 + (curr_col as i32 - next_col as i32)) as u32)?;

                    return Ok(tile_to_take == &Tile::Red || tile_to_take == &Tile::RedRoyals);
                }
                else {
                    return Ok((curr_row as i32 - next_row as i32) == 1 &&
                            (curr_col as i32 - next_col as i32).abs() == 1)
                }
            }
        }
    }

    fn get_taken_piece(&self, curr_row: u32, curr_col: u32, next_row: u32,
        next_col: u32) -> Option<(u32, u32)> {
        if (curr_row as i32 - next_row as i32).abs() == 2 && 
               (curr_col as i32 - next_col as i32).abs() == 2 {
            return Some(((curr_row as i32 + (curr_row as i32 - next_row as i32)) as u32,
                    (curr_col as i32 + (curr_col as i32 - next_col as i32)) as u32))        
        }
        return None;
    }

}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_str: String = "".to_owned();
        for row in &self.data {
            for tile in row {
                board_str.push_str(match tile {
                    Tile::Empty => ". ",
                    Tile::Black => "+ ",
                    Tile::Red => "x ",
                    Tile::BlackRoyals => "# ",
                    Tile::RedRoyals => "X ",
                })
            }
            board_str.push_str("\n")
        }
        write!(f, "{}", board_str)
    }
}

pub struct GameState {
    pub boards: Vec<BoardState>,
    pub red_turn: bool
}



// Given a position like "c5" returns the column that the alphanumeric character corresponds to.
// In this case c would be 3. It is not case sensitive
fn pos_to_col(pos: String) -> Result<u32, String> {
    // I have some questions about the proper way to substring here
    if pos.chars().count() < 2 {
        return Err("Position is too short.".to_owned());
    }
    let c = pos.chars().next().unwrap() as u32;
    if c > 64 && c < 97 {
        return Ok(c - 65);
    }
    else if c > 96 && c < 123{
        return Ok(c - 97);
    }
    else {
        return Err("First character out of scope. Use the alphabet.".to_owned());
    }

}
 
// Given a position like "c5" returns the row that the number corresponds to.
fn pos_to_row(pos: String) -> Result<u32, String> {
    if pos.chars().count() < 2 {
        return Err("Position is too short.".to_owned());
    }
    let mut chars = pos.chars();
    chars.next();            red_turn = !red_turn;
    // need to add logic for if not a digit
    return Ok(chars.as_str().to_owned().parse::<u32>().unwrap() - 1);
}