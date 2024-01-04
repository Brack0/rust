mod game {
    use std::fmt;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Player {
        X,
        O,
    }

    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Player::X => write!(f, "X"),
                Player::O => write!(f, "O"),
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    struct Cell(Option<Player>);

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Cell(Some(p)) => write!(f, "{:?}", p),
                Cell(None) => write!(f, " "),
            }
        }
    }

    pub struct Board {
        cells: [[Cell; 3]; 3],
        pub current_player: Player,
    }

    impl Board {
        pub fn play(&mut self, position: usize) -> Result<(), &'static str> {
            get_row_col_from_position(position).and_then(|(row, col)| {
                if self.cells[row][col] != Cell(None) {
                    Err("Invalid move. The cell is not empty.")
                } else {
                    self.cells[row][col] = Cell(Some(self.current_player));
                    self.current_player = get_next_player(self.current_player);
                    Ok(())
                }
            })
        }

        pub fn get_winner(&self) -> Option<Player> {
            for i in 0..3 {
                if self.cells[i][0] == self.cells[i][1] && self.cells[i][1] == self.cells[i][2] {
                    if let Cell(Some(player)) = self.cells[i][0] {
                        return Some(player);
                    }
                }
                if self.cells[0][i] == self.cells[1][i] && self.cells[1][i] == self.cells[2][i] {
                    if let Cell(Some(player)) = self.cells[0][i] {
                        return Some(player);
                    }
                }
            }

            if self.cells[0][0] == self.cells[1][1] && self.cells[1][1] == self.cells[2][2] {
                if let Cell(Some(player)) = self.cells[0][0] {
                    return Some(player);
                }
            }
            if self.cells[0][2] == self.cells[1][1] && self.cells[1][1] == self.cells[2][0] {
                if let Cell(Some(player)) = self.cells[0][2] {
                    return Some(player);
                }
            }

            None
        }

        pub fn is_draw(&self) -> bool {
            self.cells
                .iter()
                .all(|row| row.iter().all(|Cell(cell_content)| cell_content.is_some()))
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "┌───┬───┬───┐\n\
                 │ {} │ {} │ {} │\n\
                 ├───┼───┼───┤\n\
                 │ {} │ {} │ {} │\n\
                 ├───┼───┼───┤\n\
                 │ {} │ {} │ {} │\n\
                 └───┴───┴───┘",
                self.cells[0][0],
                self.cells[0][1],
                self.cells[0][2],
                self.cells[1][0],
                self.cells[1][1],
                self.cells[1][2],
                self.cells[2][0],
                self.cells[2][1],
                self.cells[2][2]
            )
        }
    }

    pub fn new_board() -> Board {
        Board {
            cells: [[Cell(None); 3]; 3],
            current_player: Player::X,
        }
    }

    fn get_next_player(player: Player) -> Player {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn get_row_col_from_position(position: usize) -> Result<(usize, usize), &'static str> {
        if position > 0 && position < 10 {
            Ok((((9 - position) / 3), ((position - 1) % 3)))
        } else {
            Err("Wrong position. Please enter a value between 1 and 9.")
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn row_col_from_wrong_position() {
            assert_eq!(get_row_col_from_position(0), Err("Wrong position"));
            assert_eq!(get_row_col_from_position(10), Err("Wrong position"));
            assert_eq!(get_row_col_from_position(1000), Err("Wrong position"));
        }

        #[test]
        fn row_col_from_right_position() {
            assert_eq!(get_row_col_from_position(1), Ok((2, 0)));
            assert_eq!(get_row_col_from_position(2), Ok((2, 1)));
            assert_eq!(get_row_col_from_position(3), Ok((2, 2)));
            assert_eq!(get_row_col_from_position(4), Ok((1, 0)));
            assert_eq!(get_row_col_from_position(5), Ok((1, 1)));
            assert_eq!(get_row_col_from_position(6), Ok((1, 2)));
            assert_eq!(get_row_col_from_position(7), Ok((0, 0)));
            assert_eq!(get_row_col_from_position(8), Ok((0, 1)));
            assert_eq!(get_row_col_from_position(9), Ok((0, 2)));
        }
    }
}

fn main() {
    print!(
        "Place your mark by selecting a number\n\
         ┌───┬───┬───┐\n\
         │ 7 │ 8 │ 9 │\n\
         ├───┼───┼───┤\n\
         │ 4 │ 5 │ 6 │\n\
         ├───┼───┼───┤\n\
         │ 1 │ 2 │ 3 │\n\
         └───┴───┴───┘\n"
    );
    let mut board = game::new_board();
    loop {
        println!("Current Board:");
        println!("{}", board);
        println!("Player {:?}, enter your move", board.current_player);
        let position = match std::io::stdin()
            .lines()
            .next()
            .expect("end of input")
            .expect("error reading from stdin")
            .trim()
            .parse::<usize>()
        {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        match board.play(position) {
            Ok(_) => {
                if let Some(winner) = board.get_winner() {
                    println!("{}", board);
                    println!("Player {:?} wins!", winner);
                    break;
                } else if board.is_draw() {
                    println!("{}", board);
                    println!("It's a draw!");
                    break;
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
