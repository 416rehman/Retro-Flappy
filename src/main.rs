use bracket_lib::prelude::{main_loop, BError, BTermBuilder};

mod core;
mod game_mode;
mod obstacle;
mod player;
mod state;

const FRAME_DURATION: f32 = 75.0 / 3.0;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const POSITION_ON_SCREEN: i32 = 15;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Retro Flappy")
        .build()?;

    main_loop(context, state::State::new())
}
