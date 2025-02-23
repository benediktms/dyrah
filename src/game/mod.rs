use serde::{Deserialize, Serialize};

pub mod server;
pub mod client;

#[derive(Serialize, Deserialize)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Serialize, Deserialize)]
enum ServerMessage {
    PlayerConnected { id: u64, pos: Position },
    PlayerMoved { id: u64, pos: Position }
}

#[derive(Serialize, Deserialize)]
enum ClientMessage {
    PlayerMove { x: f32, y: f32 }
}