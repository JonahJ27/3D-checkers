pub mod board;

use std::env;
use std::fs;
use std::io;
use crate::board::Tile;
use crate::board::BoardState;
use crate::board::GameState;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let mut hold_game_state = None;
    if args.len() < 2 {
        hold_game_state = gen_game_from_input();
    }
    else {
        match fs::read_to_string(&args[1]) {
            Ok(s) =>  {
                match load_game_from_file(s) {
                    Ok(game_state) => hold_game_state = Some(game_state),
                    Err(s) => {
                        println!("{}\nTrying manual board generation.", s);
                        hold_game_state = gen_game_from_input();
                    }
                }

            }
            Err(_) => {
                println!("Invalid file location");
                hold_game_state = gen_game_from_input();
            }
        };
    }
    

    if let Some(game_state) = hold_game_state {
        return play_game_loop(game_state);
    } 

    
    Ok(())
}

fn play_game_loop(mut game_state: GameState) -> io::Result<()> {
    loop {
            println!("Input board number.");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            if let Some(board_state) =  game_state.boards.get_mut(
                buffer.trim().parse::<u32>().unwrap_or_default() as usize) {
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
                match board_state.try_to_move(position_list.clone(), game_state.red_turn) {
                    Err(s) => println!("{}", s),
                    Ok(valid) => { 
                        println!("{}", valid);
                        game_state.red_turn = !game_state.red_turn;
                    }
                }
            }
            
        }
}

fn load_game_from_file(input: String) -> Result<GameState, String> {
    let mut lines : Vec<&str> = input.lines().map(|s| s.trim()).collect();
    let sizes: Vec<u32> = lines.remove(0).split(",").map(|s| s.to_string().trim()
                                .parse::<u32>().unwrap_or_default()).collect();
    let red_turn = lines.remove(0) == "true";

    let mut boards: Vec<BoardState> = Vec::new();
    for size in sizes {
        lines.remove(0);
        let mut board: Vec<Vec<Tile>> = Vec::new();
        for _ in 0..size {
            let line = lines.remove(0);
            println!("{}", line);
            let row: Result<Vec<Tile>, String> = line.split(" ").map(|s| {
                Ok(match s.to_string().trim() {
                    "." => Tile::Empty,
                    "+" => Tile::Black,
                    "x" => Tile::Red,
                    "#" => Tile::BlackRoyals,
                    "X" => Tile::RedRoyals,
                    s => {
                        return Err("File is invalid".to_owned() + s);
                    },
                })
            }).collect();
            let clean_row = row?;
            if clean_row.len() != size as usize {
                return Err("Row has wrong number pieces.".to_owned());
            }

            board.push(clean_row);
        }
        // may need reverse
        boards.push(BoardState{data: board});
    }
                            
        return Ok(GameState{boards: boards, red_turn: red_turn});
}

fn gen_game_from_input() -> Option<GameState> {
    let mut buffer = String::new();
        println!("Input board sizes separated by commas");
        // ask about this
        loop {
            io::stdin().read_line(&mut buffer).ok();
            let sizes: Vec<u32> = buffer.split(",").map(|s| s.to_string().trim()
                                        .parse::<u32>().unwrap_or_default()).collect();
            match gen_game_state(sizes) {
                Err(s) => println!("{}", s), 
                Ok(game_state) => {
                    return Some(game_state);
                }
            }

            println!("Try again. Input board sizes separated by commas");      
            buffer = String::new();
        }
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

    return Ok(GameState {boards: games, red_turn: false});
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

    let board_state = BoardState{data: board};

    return Ok(board_state);
}
