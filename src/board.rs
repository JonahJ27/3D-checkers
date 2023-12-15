use std::fmt;

use crate::board;

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
    fn try_to_move(&self, mut position_list: Vec<String>, red_turn: bool) -> Result<String, String> {
        let first_pos = position_list.pop();
        let curr_pos:String = match first_pos {
            None => return Err("Not enough positions entered for a move.".to_owned()),
            Some(s) => s,
        };

        if red_turn {
            let c = match pos_col_to_index(curr_pos.clone()) {
                Ok(i) => i,
                Err(s) => return Err(s),
            };
            let r = match pos_row_to_index(curr_pos.clone()) {
                Ok(i) => i,
                Err(s) => return Err(s),
            };
            // feels gross to do this just to redefine
            let mut is_royal = false;
            match self.data.get(r as usize) {
                None => return Err("Row index out of bounds".to_owned()),
                Some(row) => match row.get(c as usize) {
                    None => return Err("Column index out of bounds.".to_owned()),
                    Some(tile) => match tile {
                        Tile::Empty => return Err("Target position ".to_owned() + &curr_pos
                                                + " is empty."),
                        Tile::Black => return Err("Target position ".to_owned() + &curr_pos
                                                + " is black."),
                        Tile::Red => is_royal = false,
                        Tile::BlackRoyals => return Err("Target position ".to_owned() 
                                                + &curr_pos + " is empty."),
                        Tile::RedRoyals => is_royal = true,
                    }
                },
            }
        }
        else {

        }

        return Ok("Valid Move".to_owned());
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


fn pos_col_to_index(pos: String) -> Result<u32, String> {
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

fn pos_row_to_index(pos: String) -> Result<u32, String> {
    if pos.chars().count() < 2 {
        return Err("Position is too short.".to_owned());
    }
    let mut chars = pos.chars();
    chars.next();
    // need to add logic for if not a digit
    return Ok(chars.as_str().to_owned().parse::<u32>().unwrap());
}