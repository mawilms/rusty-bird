use ggez::{
    conf::{self, WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{clear, draw, present, Color, DrawParam, Image},
    timer::{self},
    Context, ContextBuilder, GameResult,
};
use rand::Rng;
use std::env;

const FRAMERATE: u32 = 60;
const TUBE_STEP_SIZE: f32 = 250.;
const GRAVITY: f32 = -0.2;
const FLAPPING: f32 = 5.;

pub struct Player {
    x: f32,
    y: f32,
    is_flapping: bool,
    assets: Vec<Image>,
}

pub struct Tube {
    x: f32,
    y: f32,
    asset_up: Image,
    asset_down: Image,
    passed: bool,
}

pub struct Game {
    player: Player,
    tubes: Vec<Tube>,
    score: u16,
    background: Image,
    vertical_speed: f32,
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FRAMERATE) {
            self.vertical_speed += GRAVITY;
            self.player.y -= self.vertical_speed;

            for tube in &mut self.tubes {
                tube.x -= 3.;
            }

            if self.player.x <= self.tubes[0].x && !self.tubes[0].passed {
                self.tubes[0].passed = true;
                self.score += 1; // TODO: Currently bugged because the first tube is always in front of the bird. Therefore the game starts at score 1 and the first tube is set to passed=true
            }

            if self.tubes[0].x <= -52. {
                let mut rng = rand::thread_rng();
                self.tubes.remove(0);
                self.tubes.push(Tube {
                    x: self.tubes.last().unwrap().x + TUBE_STEP_SIZE,
                    y: rng.gen_range(200., 400.),
                    asset_up: Image::new(ctx, "/pipe-green-up.png")?,
                    asset_down: Image::new(ctx, "/pipe-green-down.png")?,
                    passed: false,
                })
            }
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
                DrawParam::default().dest(nalgebra::Point2::new(tube.x, tube.y - 450.)),
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

        draw(ctx, &self.player.assets[0], player_draw_param).expect("Error while drawing player");

        present(ctx).expect("Error while presenting");
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Space {
            self.player.is_flapping = true;
            self.vertical_speed = FLAPPING;
        }
    }
}

impl Game {
    pub fn start() -> GameResult {
        let mut rng = rand::thread_rng();
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

        let mut tubes = vec![];
        let mut x_initial_range = rng.gen_range(600., 650.);
        for _ in 0..7 {
            tubes.push(Tube {
                x: x_initial_range,
                y: rng.gen_range(200., 400.),
                asset_up: Image::new(ctx, "/pipe-green-up.png")?,
                asset_down: Image::new(ctx, "/pipe-green-down.png")?,
                passed: false,
            });
            x_initial_range += TUBE_STEP_SIZE;
        }

        let state = &mut Game {
            player: Player {
                x: 50.,
                y: 100.,
                is_flapping: false,
                assets: vec![
                    Image::new(ctx, "/bluebird-upflap.png")?,
                    Image::new(ctx, "/bluebird-midflap.png")?,
                    Image::new(ctx, "/bluebird-downflap.png")?,
                ],
            },
            tubes,
            score: 0,
            background: Image::new(ctx, "/background-night.png")?,
            vertical_speed: 0.,
        };

        event::run(ctx, event_loop, state)
    }
}
