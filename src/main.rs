pub mod board;

use std::io;
use crate::board::Tile;
use crate::board::BoardState;
use crate::board::GameState;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    println!("{}", buffer);
    let size = buffer.trim().parse::<u32>().unwrap_or_default();
    let mut game_state: Result<BoardState, String> = gen_board(size);
    match game_state {
        Err(game_state) => println!("{}", game_state),
        Ok(game_state) => print!("{}", game_state),
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
        return Err("Checkers board must be even".to_owned())
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
