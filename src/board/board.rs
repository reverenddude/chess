use std::array;
use std::fmt;
use std::fmt::Display;

use crate::pieces::types::ChessPiece;

pub struct Board {
    state: [Square; 64],
}

pub struct Square {
    pub occupied: Option<ChessPiece>,
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(p) = &self.occupied {
            write!(f, "{}", p)
        } else {
            write!(f, " ")
        }
    }
}

impl Board {
    fn init() -> Self {
        Self {
            state: array::from_fn(|_| Square { occupied: None }),
        }
    }

    fn display(&self) {
        let mut rank: usize = 0;
        println!(" -------------------------- ");
        loop {
            if rank >= 8 {
                break;
            }

            let id0 = rank * 8;
            let idf = ((rank + 1) * 8) - 1;

            // println!("Rank: {rank}\nStart Index: {id0}\nEnd Index: {idf}");

            let row: &[Square] = &self.state[id0..idf];

            let display: String = row
                .iter()
                .map(|p| format!(" {} ", p))
                .collect::<Vec<String>>()
                .join("|");

            println!("{display}");

            rank += 1;
        }
        println!(" -------------------------- ");
    }
}

mod test {
    use super::*;
    use crate::pieces::types::{Color, Piece};

    #[test]
    fn test_display() {
        let square: Square = Square {
            occupied: Some(ChessPiece {
                piece: Piece::Bishop,
                color: Color::Black,
            }),
        };

        println!("{square}");

        let mut board: Board = Board::init();
        board.state[0] = square;
        //
        board.display();
    }
}
