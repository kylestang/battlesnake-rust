use serde::{Serialize, Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    x: i64,
    y: i64
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate {x, y}
    }

    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: String,
    ruleset: Ruleset,
    timeout: i64
}

impl Game {
    pub fn new(id: String, ruleset: Ruleset, timeout: i64) -> Game {
        Game {id, ruleset, timeout}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ruleset {
    name: String,
    version: String
}

impl Ruleset {
    pub fn new(name: String, version: String) -> Ruleset {
        Ruleset {name, version}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: i64,
    body: Vec<Coordinate>,
    latency: String,
    head: Coordinate,
    length: i64,
    shout: String,
}

impl Battlesnake {
    pub fn new(
        id: String, name: String, health: i64, body: Vec<Coordinate>, latency: String,
        head: Coordinate, length: i64, shout: String
    ) -> Battlesnake {
        Battlesnake {id, name, health, body, latency, head, length, shout}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    height: i64,
    width: i64,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>
}

impl Board {
    pub fn new(height: i64, width: i64, food: Vec<Coordinate>, hazards: Vec<Coordinate>, snakes: Vec<Battlesnake>) -> Board {
        Board {height, width, food, hazards, snakes}
    }
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct MoveRequest {
    game: Game,
    turn: i64,
    board: Board,
    you: Battlesnake
}

impl MoveRequest {
    pub fn new(game: Game, turn: i64, board: Board, you: Battlesnake) -> MoveRequest {
        MoveRequest {game, turn, board, you}
    }

    pub fn get_game(&self) -> &Game {
        &self.game
    }

    pub fn get_turn(&self) -> i64 {
        self.turn
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_you(&self) -> &Battlesnake {
        &self.you
    }
}

#[derive(Serialize)]
pub struct IndexResponse {
    apiversion: &'static str,
    author: &'static str,
    color: &'static str,
    head: &'static str,
    tail: &'static str,
}

impl IndexResponse {
    pub fn new(
        apiversion: &'static str, author: &'static str,
        color: &'static str, head: &'static str, tail: &'static str
    ) -> IndexResponse {
        IndexResponse {apiversion, author, color, head, tail}
    }
}

#[derive(Serialize)]
pub struct MoveResponse {
    r#move: String,
    shout:String
}

impl MoveResponse {
    pub fn new(r#move: String, shout: String) -> MoveResponse {
        MoveResponse {r#move, shout}
    }
}
