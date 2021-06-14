use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Packet {
    player: f32,
    score: i32,
    pipes: Vec<((f32, f32), (f32, f32))>,
}

impl Packet {
    pub fn new(player: f32, score: i32, pipes: Vec<((f32, f32), (f32, f32))>) -> Self {
        Self {
            player,
            score,
            pipes,
        }
    }
}
