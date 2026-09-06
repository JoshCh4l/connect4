use crossterm::{
    cursor, execute,
    style::{Color, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

use crate::player::Player;

#[derive(Debug)]
pub struct Matrix {
    matrix: [[u32; 7]; 6],
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            matrix: [[0; 7]; 6],
        }
    }

    pub fn print(&self, player: &Player) {
        let mut stdout = io::stdout();

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(ClearType::All)
        )
        .unwrap();

        match player {
            Player::One => writeln!(stdout, "Player 1's turn").unwrap(),
            Player::Two => writeln!(stdout, "Player 2's turn").unwrap(),
        }
        execute!(stdout, cursor::MoveTo(0, 2)).unwrap();
        writeln!(stdout, "1 2 3 4 5 6 7").unwrap();

        execute!(stdout, cursor::MoveTo(0, 3)).unwrap();
        writeln!(stdout, "-------------").unwrap();

        for (y, row) in self.matrix.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, y as u16 + 4)).unwrap();

            for value in row {
                match value {
                    1 => execute!(stdout, SetForegroundColor(Color::Red)).unwrap(),
                    2 => execute!(stdout, SetForegroundColor(Color::Green)).unwrap(),
                    _ => execute!(stdout, SetForegroundColor(Color::White)).unwrap(),
                }
                write!(stdout, "{value} ").unwrap();
            }
        }

        stdout.flush().unwrap();
    }

    pub fn put_token1(&mut self, x: usize) -> bool {
        for row in self.matrix.iter_mut().rev() {
            if row[x] == 0 {
                row[x] = 1;
                return true;
            };
        }
        false
    }

    pub fn put_token2(&mut self, x: usize) -> bool {
        for row in self.matrix.iter_mut().rev() {
            if row[x] == 0 {
                row[x] = 2;
                return true;
            };
        }
        false
    }

    pub fn check_winner(&self, player: u32) -> bool {
        // Horizontal
        for row in 0..6 {
            for col in 0..4 {
                if self.matrix[row][col] == player
                    && self.matrix[row][col + 1] == player
                    && self.matrix[row][col + 2] == player
                    && self.matrix[row][col + 3] == player
                {
                    return true;
                }
            }
        }

        // Vertical
        for row in 0..3 {
            for col in 0..7 {
                if self.matrix[row][col] == player
                    && self.matrix[row + 1][col] == player
                    && self.matrix[row + 2][col] == player
                    && self.matrix[row + 3][col] == player
                {
                    return true;
                }
            }
        }

        // Diagonal ↘
        for row in 0..3 {
            for col in 0..4 {
                if self.matrix[row][col] == player
                    && self.matrix[row + 1][col + 1] == player
                    && self.matrix[row + 2][col + 2] == player
                    && self.matrix[row + 3][col + 3] == player
                {
                    return true;
                }
            }
        }

        // Diagonal ↗
        for row in 3..6 {
            for col in 0..4 {
                if self.matrix[row][col] == player
                    && self.matrix[row - 1][col + 1] == player
                    && self.matrix[row - 2][col + 2] == player
                    && self.matrix[row - 3][col + 3] == player
                {
                    return true;
                }
            }
        }

        false
    }
}
