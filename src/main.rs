use crossterm::{
    event::{self, Event},
    terminal::{self},
};
use player::Player;
use std::{
    io::{self},
    ops::ControlFlow,
};

mod input;
mod matrix;
mod player;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut mtx = matrix::Matrix::new();
    let mut current_player = Player::One;

    loop {
        mtx.print(&current_player, false);

        if let Event::Key(key) = event::read()? {
            if let ControlFlow::Break(_) = input::player_input(&mut mtx, &mut current_player, key) {
                break;
            } else {
                continue;
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
