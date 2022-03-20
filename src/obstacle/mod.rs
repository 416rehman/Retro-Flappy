use crate::player::Player;
use crate::{POSITION_ON_SCREEN, SCREEN_HEIGHT};
use bracket_lib::prelude::{to_cp437, BTerm, RandomNumberGenerator, BLACK, RED};

pub struct Obstacle {
    pub x: i32,
    gap_center: i32,
    gap_length: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_center: random.range(10, 40),
            gap_length: i32::max(2, 20 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x; // World to screen
        let half_length = self.gap_length / 2;

        //from top of screen to top of gap
        for y in 0..self.gap_center - half_length {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }

        for y in self.gap_center + half_length..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'))
        }
    }

    pub fn collided(&self, player: &Player) -> bool {
        let half_size = self.gap_length / 2;
        let does_x_match = (player.position.x.value + POSITION_ON_SCREEN) == self.x;

        let is_above_gap = player.position.y.value < (self.gap_center - half_size);
        let is_below_gap = player.position.y.value > (self.gap_center + half_size);

        does_x_match && (is_below_gap || is_above_gap)
    }
}
