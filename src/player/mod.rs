use crate::core::Vector2;
use crate::{POSITION_ON_SCREEN};
use bracket_lib::prelude::{to_cp437, BTerm, BLACK, TURQUOISE};

pub struct Player {
    pub(crate) position: Vector2<i32>,
    velocity: f32,
}

impl Player {
    pub fn new(pos: Vector2<i32>) -> Self {
        Self {
            position: pos,
            velocity: 0.0,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            POSITION_ON_SCREEN,
            self.position.y.value as i32,
            TURQUOISE,
            BLACK,
            to_cp437('@'),
        );
    }

    pub fn gravity_and_move(&mut self) {
        if self.velocity < 1.0 {
            self.velocity += 0.1;
        }

        self.position.y.value += self.velocity as i32;
        self.position.x.value += 1;

        if self.position.y.value < 0 {
            self.position.y.set(0);
        }
    }

    /// Move upwards
    pub fn flap(&mut self) {
        self.velocity = -1.7;
    }
}
