use ggez::{
    conf::{self, WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{clear, draw, present, Color, DrawParam, Image},
    timer, Context, ContextBuilder, GameResult,
};
use std::env;

pub struct Player {
    x: f32,
    y: f32,
    velocity: f32,
    asset_one: Image,
    asset_two: Image,
    asset_three: Image,
}

pub struct Tube {
    x: f32,
    y: f32,
    asset_up: Image,
    asset_down: Image,
}

pub struct Game {
    player: Player,
    tubes: Vec<Tube>,
    score: u16,
    framerate: std::time::Duration,
    background: Image,
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.framerate = timer::delta(ctx);
        self.player.y += self.player.velocity;

        for tube in &mut self.tubes {
            tube.x -= 3.;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0., 0., 0., 1.));

        draw(
            ctx,
            &self.background,
            DrawParam::default().dest(nalgebra::Point2::new(0., 0.)),
        )?;
        draw(
            ctx,
            &self.background,
            DrawParam::default().dest(nalgebra::Point2::new(288., 0.)),
        )?;
        draw(
            ctx,
            &self.background,
            DrawParam::default().dest(nalgebra::Point2::new(576., 0.)),
        )?;

        for tube in &self.tubes {
            draw(
                ctx,
                &tube.asset_down,
                DrawParam::default().dest(nalgebra::Point2::new(tube.x, -200.)),
            )
            .expect("Error while drawing tubes");
            draw(
                ctx,
                &tube.asset_up,
                DrawParam::default().dest(nalgebra::Point2::new(tube.x, tube.y)),
            )
            .expect("Error while drawing tubes");
        }

        let player_draw_param =
            DrawParam::new().dest(nalgebra::Point2::new(self.player.x, self.player.y));
        draw(ctx, &self.player.asset_one, player_draw_param).expect("Error while drawing player");

        present(ctx).expect("Error while presenting");
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Space {
            self.player.y -= 100.;
        }
    }
}

impl Game {
    pub fn start() -> GameResult {
        let resource_path = env::current_dir()
            .unwrap()
            .join("src")
            .join("game")
            .join("assets");

        let mut config = conf::Conf::new();
        config.window_mode = WindowMode::default().dimensions(864., 512.);
        config.window_setup = WindowSetup::default().title("Rusty Bird");

        let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Rusty Bird", "Marius Wilms")
            .conf(config)
            .add_resource_path(resource_path)
            .build()?;

        let tubes = vec![Tube {
            x: 650.,
            y: 200.,
            asset_up: Image::new(ctx, "/pipe-green-up.png")?,
            asset_down: Image::new(ctx, "/pipe-green-down.png")?,
        }];

        let state = &mut Game {
            player: Player {
                x: 50.,
                y: 300.,
                velocity: 2.,
                asset_one: Image::new(ctx, "/bluebird-upflap.png")?,
                asset_two: Image::new(ctx, "/bluebird-midflap.png")?,
                asset_three: Image::new(ctx, "/bluebird-downflap.png")?,
            },
            tubes,
            score: 0,
            framerate: std::time::Duration::new(0, 0),
            background: Image::new(ctx, "/background-night.png")?,
        };

        event::run(ctx, event_loop, state)
    }
}
