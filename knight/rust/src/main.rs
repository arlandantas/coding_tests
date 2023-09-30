/*
3
...
...
K..
*/

use std::{io, collections::HashMap};

#[derive(Debug,Hash,Clone)]
struct Position {
    x: i32,
    y: i32
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

type Board = Vec<Vec<String>>;

#[derive(Debug)]
struct BoardInput {
    board: Board,
    horse_position: Position
}

const BOARD_CELL_KNIGHT: &str = "K";
const BOARD_CELL_EMPTY: &str = ".";
const BOARD_CELL_BLOCKED: &str = "#";

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}

fn read_usize() -> usize {
    return read_string().parse().expect("Failed to parse input to number");
}

fn read_board() -> BoardInput {
    let board_size = read_usize();
    let mut board: Board = vec![vec![BOARD_CELL_EMPTY.to_string(); board_size]; board_size];
    let mut horse_position = Position{ x: 0, y: 0 };

    for i in 0..board_size {
        let line = read_string();
        let mut j: usize = 0;
        for char in line.chars() {
            board[i][j] = char.to_string();
            if board[i][j] == BOARD_CELL_KNIGHT {
                horse_position = Position { x: j as i32, y: i as i32 };
            }
            j += 1;
            if j >= board_size {
                break;
            }
        }
    }

    return BoardInput {
        board,
        horse_position
    };
}

fn get_position_targets(origin: &Position, board_size: &i32) -> Vec<Position> {
    let mut positions:Vec<Position> = vec![
        Position { x: origin.x - 2, y: origin.y - 1 },
        Position { x: origin.x - 2, y: origin.y + 1 },
        Position { x: origin.x + 2, y: origin.y - 1 },
        Position { x: origin.x + 2, y: origin.y + 1 },
        Position { x: origin.x - 1, y: origin.y - 2 },
        Position { x: origin.x - 1, y: origin.y + 2 },
        Position { x: origin.x + 1, y: origin.y - 2 },
        Position { x: origin.x + 1, y: origin.y + 2 },
    ];
    positions.retain(|position| position.x >= 0 && position.y >= 0 && position.x < *board_size && position.y < *board_size);
    return positions;
}

fn count_moviments_to(board: Board, horse_position: Position, target_position: Position) -> i32 {
    let mut visited_places: HashMap<Position, i32> = HashMap::new();
    visited_places.insert(horse_position.clone(), 0);

    let board_size = board.len() as i32;

    let mut queue = vec![horse_position];
    
    while queue.len() > 0 {
        let position = queue
            .remove(0);
        
        let current_steps = visited_places
            .get(&position)
            .expect("Failed to get current steps")
            .to_owned();
        
        if position == target_position {
            return current_steps;
        }

        for future_position in get_position_targets(&position, &board_size) {
            if visited_places.contains_key(&future_position) || queue.contains(&future_position) {
                continue;
            }
            
            if board[future_position.y as usize][future_position.x as usize] == BOARD_CELL_BLOCKED {
                continue;
            }

            visited_places.insert(future_position.clone(), current_steps + 1);
            queue.push(future_position);
        }
    }
    return -1;
}

fn main() {
    let user_input = read_board();
    let moviments = count_moviments_to(user_input.board, user_input.horse_position, Position {x:0,y:0});
    println!("{:?}", moviments);
}