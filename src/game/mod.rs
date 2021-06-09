mod pipe;
mod player;

use ggez::{
    conf::{self, WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{clear, draw, present, Color, DrawParam, Image, Rect, Text, WHITE},
    timer::{self},
    Context, ContextBuilder, GameResult,
};
use rand::Rng;
use std::collections::VecDeque;

const FRAMERATE: u32 = 60;
const TUBE_STEP_SIZE: f32 = 250.;
const GRAVITY: f32 = -0.2;
const FLAPPING: f32 = 5.;

pub struct Game {
    player: player::Player,
    tubes: VecDeque<(pipe::Pipe, pipe::Pipe)>,
    score: i32,
    background: Image,
    vertical_speed: f32,
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, FRAMERATE) {
            self.vertical_speed += GRAVITY;
            self.player.rect.y -= self.vertical_speed;

            // Environmental rules like upper and lower level bounds
            if self.player.rect.top() <= 0. || self.player.rect.bottom() >= 512. {
                self.restart_game(ctx);
            }

            // Tube movement
            for tube in &mut self.tubes {
                tube.0.rect.x -= 3.;
                tube.1.rect.x -= 3.;
            }

            // Scoring
            if self.player.rect.x <= self.tubes[0].0.rect.x && !self.tubes[0].0.passed {
                self.tubes[0].0.passed = true;
                self.score += 1; // TODO: Currently bugged because the first tube is always in front of the bird. Therefore the game starts at score 1 and the first tube is set to passed=true
            }

            // Collision detection
            if self.player.rect.overlaps(&self.tubes[0].0.rect)
                || self.player.rect.overlaps(&self.tubes[0].1.rect)
            {
                self.restart_game(ctx);
            }

            // Delete pipes that are out of view
            if self.tubes[0].0.rect.x <= -52. {
                let mut rng = rand::thread_rng();
                let y_position = rng.gen_range(200..400) as f32;
                self.tubes.pop_front();
                self.tubes.push_back((
                    pipe::Pipe::new(
                        ctx,
                        self.tubes.back().unwrap().0.rect.x + TUBE_STEP_SIZE,
                        y_position,
                    ),
                    pipe::Pipe::new(
                        ctx,
                        self.tubes.back().unwrap().0.rect.x + TUBE_STEP_SIZE,
                        y_position - 450.,
                    ),
                ));
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        clear(ctx, Color::new(0., 0., 0., 1.));

        draw(
            ctx,
            &self.background,
            DrawParam::default().dest(glam::Vec2::new(0., 0.)),
        )?;
        draw(
            ctx,
            &self.background,
            DrawParam::default().dest(glam::Vec2::new(288., 0.)),
        )?;
        draw(
            ctx,
            &self.background,
            DrawParam::default().dest(glam::Vec2::new(576., 0.)),
        )?;

        for tube in &self.tubes {
            draw(
                ctx,
                &tube.1.asset_down,
                DrawParam::default().dest(glam::Vec2::new(tube.1.rect.x, tube.1.rect.y)),
            )
            .expect("Error while drawing tubes");
            draw(
                ctx,
                &tube.0.asset_up,
                DrawParam::default().dest(glam::Vec2::new(tube.0.rect.x, tube.0.rect.y)),
            )
            .expect("Error while drawing tubes");
        }

        let player_draw_param =
            DrawParam::new().dest(glam::Vec2::new(self.player.rect.x, self.player.rect.y));

        draw(ctx, &self.player.assets[0], player_draw_param).expect("Error while drawing player");

        let fps = timer::fps(ctx);
        let fps_display = Text::new(format!("FPS: {}", fps));
        draw(ctx, &fps_display, (glam::Vec2::new(0.0, 10.0), WHITE))?;

        let score_display = Text::new(format!("Score: {}", self.score - 1));
        draw(ctx, &score_display, (glam::Vec2::new(432., 10.0), WHITE))?;

        present(ctx).expect("Error while presenting");
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        if keycode == KeyCode::Space {
            self.vertical_speed = FLAPPING;
        }
    }
}

impl Game {
    pub fn start() -> GameResult {
        let background_img =
            image::load_from_memory(include_bytes!("./assets/background-night.png"))
                .expect("loading icon")
                .to_rgba8();
        let (bg_width, bg_height) = background_img.dimensions();

        let mut rng = rand::thread_rng();

        let mut config = conf::Conf::new();
        config.window_mode = WindowMode::default().dimensions(864., 512.);
        config.window_setup = WindowSetup::default().title("Rusty Bird");

        let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Rusty Bird", "Marius Wilms")
            .conf(config)
            .build()?;

        let mut tubes = VecDeque::new();
        let mut x_initial_range = rng.gen_range(600..650) as f32;
        for _ in 0..7 {
            let y_position = rng.gen_range(200..400) as f32;
            tubes.push_back((
                pipe::Pipe::new(ctx, x_initial_range, y_position),
                pipe::Pipe::new(ctx, x_initial_range, y_position - 450.),
            ));
            x_initial_range += TUBE_STEP_SIZE;
        }

        let state = &mut Game {
            player: player::Player::new(ctx),
            tubes,
            score: 0,
            background: Image::from_rgba8(ctx, bg_width as u16, bg_height as u16, &background_img)?,
            vertical_speed: 0.,
        };

        event::run(ctx, event_loop, state)
    }

    fn restart_game(&mut self, ctx: &mut Context) {
        let mut rng = rand::thread_rng();
        let mut x_initial_range = rng.gen_range(600..650) as f32;
        self.tubes.clear();
        for _ in 0..7 {
            let y_position = rng.gen_range(200..400) as f32;
            self.tubes.push_back((
                pipe::Pipe::new(ctx, x_initial_range, y_position),
                pipe::Pipe::new(ctx, x_initial_range, y_position - 450.),
            ));
            x_initial_range += TUBE_STEP_SIZE;
        }

        self.player.rect = Rect::new(50., 100., 34., 24.);
        self.score = 0;
    }
}
