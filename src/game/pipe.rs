use ggez::{
    graphics::{Image, Rect},
    Context,
};

pub struct Pipe {
    pub rect: Rect,
    pub asset_up: Image,
    pub asset_down: Image,
    pub passed: bool,
}

impl Pipe {
    pub fn new(ctx: &mut Context, x_pos: f32, y_pos: f32) -> Self {
        let pipe_up_img = image::load_from_memory(include_bytes!("./assets/pipe-green-up.png"))
            .expect("loading icon")
            .to_rgba();

        let pipe_down_img = image::load_from_memory(include_bytes!("./assets/pipe-green-down.png"))
            .expect("loading icon")
            .to_rgba();
        let (pipe_width, pipe_height) = pipe_down_img.dimensions();

        Self {
            rect: Rect::new(x_pos, y_pos, 52., 320.),
            asset_up: Image::from_rgba8(ctx, pipe_width as u16, pipe_height as u16, &pipe_up_img)
                .expect("Error while creating image"),
            asset_down: Image::from_rgba8(
                ctx,
                pipe_width as u16,
                pipe_height as u16,
                &pipe_down_img,
            )
            .expect("Error while creating image"),
            passed: false,
        }
    }
}
