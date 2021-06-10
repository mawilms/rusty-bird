use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Packet {
    player: (f32, f32),
    score: i32,
    tubes: Vec<((f32, f32), (f32, f32))>,
}

impl Packet {
    pub fn new(player: (f32, f32), score: i32, tubes: Vec<((f32, f32), (f32, f32))>) -> Self {
        Self {
            player,
            score,
            tubes,
        }
    }
}
