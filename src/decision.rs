use crate::structures::{Coordinate, Game, Battlesnake, Board, MoveResponse};

pub fn decision(game: &Game, turn: i64, board: &Board, you: &Battlesnake) -> MoveResponse {
    println!("Decision");
    MoveResponse::new(String::from("left"), String::from("Hi!"))
}
