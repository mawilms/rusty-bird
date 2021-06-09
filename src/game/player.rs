use ggez::{
    graphics::{Image, Rect},
    Context,
};

pub struct Player {
    pub rect: Rect,
    pub assets: Vec<Image>,
}

impl Player {
    pub fn new(ctx: &mut Context) -> Self {
        let player_up_img = image::load_from_memory(include_bytes!("./assets/bluebird-upflap.png"))
            .expect("loading icon")
            .to_rgba8();
        let player_mid_img =
            image::load_from_memory(include_bytes!("./assets/bluebird-midflap.png"))
                .expect("loading icon")
                .to_rgba8();
        let player_down_img =
            image::load_from_memory(include_bytes!("./assets/bluebird-downflap.png"))
                .expect("loading icon")
                .to_rgba8();
        let (player_width, player_height) = player_up_img.dimensions();

        Self {
            rect: Rect::new(50., 100., 34., 24.),
            assets: vec![
                Image::from_rgba8(
                    ctx,
                    player_width as u16,
                    player_height as u16,
                    &player_up_img,
                )
                .expect("Error while creating images"),
                Image::from_rgba8(
                    ctx,
                    player_width as u16,
                    player_height as u16,
                    &player_mid_img,
                )
                .expect("Error while creating images"),
                Image::from_rgba8(
                    ctx,
                    player_width as u16,
                    player_height as u16,
                    &player_down_img,
                )
                .expect("Error while creating images"),
            ],
        }
    }
}
