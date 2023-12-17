pub mod board;

use std::io;
use crate::board::Tile;
use crate::board::BoardState;
use crate::board::GameState;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    println!("Input board size");
    io::stdin().read_line(&mut buffer)?;
    let mut size = buffer.trim().parse::<u32>().unwrap_or_default();
    // ask about this
    let mut hold_board_state = None;
    loop {
        match gen_board(size) {
            Err(msg) => println!("{}", msg),
            Ok(board) => {
                hold_board_state = Some(board);
                break;
            },
        }
        buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        size = buffer.trim().parse::<u32>().unwrap_or_default();
    }
    let mut red_turn = false;
    if let Some(mut board_state) = hold_board_state {
        loop {
            println!("{}", board_state);
            let mut position_list: Vec<String> = Vec::new();
            println!("Input position of piece");
            buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            position_list.push(buffer.trim().to_owned().clone());
            loop {
                println!("Input position to move piece or enter to end");
                buffer = String::new();
                io::stdin().read_line(&mut buffer)?;
                if buffer.len() < 2 {
                    break;
                }

                position_list.push(buffer.trim().to_owned().clone());
            }
            position_list.reverse();
            match board_state.try_to_move(position_list.clone(), red_turn) {
                Err(s) => println!("{}", s),
                Ok(valid) => { 
                    println!("{}", valid);
                    red_turn = !red_turn;
                }
            }
        }
    }
    

    
    Ok(())
}



fn gen_game_state(board_sizes: Vec<u32>) -> Result<GameState, String> {
    let mut games: Vec<BoardState> = Vec::new();
    for size in board_sizes.iter() {
        let mut board = gen_board(*size);
        match board {
            Ok(board) => games.push(board),
            Err(board) => return Err(board),
        }
    }

    return Ok(GameState {boards: games, red_turn: true});
}


fn gen_board(size: u32) -> Result<BoardState, String> {
    if size % 2 == 1 {
        return Err("Checkers board must be even. Try again".to_owned())
    }

    let mut board = Vec::new();
    for r in (0..size).rev() {
        let mut row: Vec<Tile> = Vec::new();
       
        for c in (0..size).rev() {
    if (r == size / 2) ||  (r + 1 == size / 2) ||  (r + c) % 2 == 0 { 
                row.push(Tile::Empty);
            }
            else if r < size / 2 {
                row.push(Tile::Black);
            }
            else {
                row.push(Tile::Red);
            }
        }

        board.push(row);
    }

    let mut board_state = BoardState{data: board};

    return Ok(board_state);
}
