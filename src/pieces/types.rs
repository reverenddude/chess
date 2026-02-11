use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    pub piece: Piece,
    pub color: Color,
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.piece)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pawn => write!(f, "P"),
            Self::Knight => write!(f, "N"),
            Self::Bishop => write!(f, "B"),
            Self::Rook => write!(f, "R"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}
