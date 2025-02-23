use bincode::{deserialize, serialize};
use macroquad::prelude::*;
use secs::prelude::{ExecutionMode, World};
use wrym::{client::{Client, ClientEvent}, transport::LaminarTransport};

use super::{ClientMessage, Position, ServerMessage};

pub struct Game {
    client: Client<LaminarTransport>,
    world: World,
    player_id: Option<u64>
}

fn render_system(world: &World) {
    for (_, (pos,)) in world.query::<(&Position,)>() {
        draw_rectangle(pos.x, pos.y, 32., 32., RED);
    }
}

impl Game {
    pub fn new() -> Self {
        let transport = LaminarTransport::new("127.0.0.1:0");
        let mut world = World::default();

        world.add_system(render_system, ExecutionMode::Parallel);

        Self {
            client: Client::new(transport, "127.0.0.1:8080"),
            world,
            player_id: None
        }
    }

    fn update(&mut self) {
        self.client.poll();

        while let Some(event) = self.client.recv_event() {
            match event {
                ClientEvent::Connected => {
                    println!("Connected to server!!");
                }
                ClientEvent::Disconnected => {
                    println!("Lost connection to server");
                }
                ClientEvent::MessageReceived(bytes) => {
                    let server_msg = deserialize::<ServerMessage>(&bytes).unwrap();

                    match server_msg {
                        ServerMessage::PlayerConnected { id, pos } => {
                            self.world.spawn((pos,));

                            if self.player_id.is_none() {
                                self.player_id = Some(id);
                            }
                        }
                        ServerMessage::PlayerMoved { id, pos } => {
                            for (entity, (position,)) in self.world.query::<(&mut Position,)>() {
                                if entity.to_bits() == id {
                                    position.x = pos.x;
                                    position.y = pos.y;
                                }
                            }
                        }
                    }
                }
            }
        }

        let (mut move_x, mut move_y) = (0., 0.);

        if is_key_down(KeyCode::Right) {
            move_x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            move_x -= 1.0;
        }
        if is_key_down(KeyCode::Up) {
            move_y -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            move_y += 1.0;
        }
        
        if move_x != 0.0 || move_y != 0.0 {
            let msg = ClientMessage::PlayerMove { x: move_x, y: move_y };
            
            self.client.send(&serialize(&msg).unwrap());
        }
    }
    
    pub async fn run(&mut self) {
        loop {
            clear_background(SKYBLUE);

            self.update();
            self.world.run_systems();

            next_frame().await;
        }
    }
}