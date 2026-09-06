use crate::matrix;
use crate::player::Player;
use crossterm::event::{self, KeyCode};
use std::ops::ControlFlow;

pub fn player_input(
    mtx: &mut matrix::Matrix,
    current_player: &mut Player,
    key: event::KeyEvent,
    font_size: &mut i32,
) -> ControlFlow<()> {
    match key.code {
        KeyCode::Char(c @ '1'..='7') => {
            let column = c.to_digit(10).unwrap() as usize - 1;

            let placed = match current_player {
                Player::One => mtx.put_token1(column),
                Player::Two => mtx.put_token2(column),
            };

            if placed {
                if mtx.check_winner(current_player.value()) {
                    mtx.print(current_player, true);
                    return ControlFlow::Break(());
                }
                current_player.switch();
            }
        }

        KeyCode::Char('=') => {
            *font_size += 2;

            std::process::Command::new("kitty")
                .args(["@", "set-font-size", &font_size.to_string()])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }

        KeyCode::Char('-') => {
            *font_size -= 2;

            std::process::Command::new("kitty")
                .args(["@", "set-font-size", &font_size.to_string()])
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }

        KeyCode::Esc => {
            return ControlFlow::Break(());
        }

        _ => {}
    }

    ControlFlow::Continue(())
}
