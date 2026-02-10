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
            write!(f, "   ")
        }
    }
}

impl Board {
    fn display(&self) {
        let mut rank: usize = 0;
        println!(" --- --- --- --- --- --- --- --- ");
        loop {
            if rank >= 9 {
                break;
            }

            let row: &[Square] = &self.state[(rank * 8)..((rank + 1) * 8) - 1];

            rank += 1;
        }
        println!(" --- --- --- --- --- --- --- --- ");
    }
}

mod test {
    use super::*;
    use crate::pieces::types::{Color, Piece};

    #[test]
    fn test_display() {
        let piece: Square = Square {
            occupied: Some(ChessPiece {
                piece: Piece::Bishop,
                color: Color::Black,
            }),
        };

        println!("{piece}");
    }
}
