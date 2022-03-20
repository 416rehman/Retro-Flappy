use crate::core::Vector2;
use crate::game_mode::GameMode;
use crate::player::Player;

use crate::obstacle::Obstacle;
use crate::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::prelude::{BTerm, GameState, VirtualKeyCode, BLACK};

pub struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    score: i32,
    obstacle: Obstacle,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            player: Player::new(Vector2::new(5, 25)),
            frame_time: 0.0,
            score: 0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print_centered(0, "Press SPACE to flap.");
        ctx.print_centered(5, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.position.x.value);

        if self.player.position.x.value > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.position.x.value + SCREEN_WIDTH, self.score);
        }

        if (self.player.position.y.value) > SCREEN_HEIGHT || self.obstacle.collided(&self.player) {
            self.mode = GameMode::End;
        }
    }

    pub fn restart(&mut self) {
        self.mode = GameMode::Playing;
        self.player = Player::new(Vector2::new(5, 25));
        self.frame_time = 0.0;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, self.score);
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Retro Flappy");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(15, "You are dead!");
        ctx.print_centered(17, &format!("Your score: {}", self.score));

        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}
