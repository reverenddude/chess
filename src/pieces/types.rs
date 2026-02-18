use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct ChessPiece {
    pub piece: Piece,
    pub color: Color,
}

impl Display for ChessPiece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.color {
            Color::White => write!(f, "{}", self.piece.as_str().to_uppercase()),
            Color::Black => write!(f, "{}", self.piece.as_str()),
        }
    }
}

impl ChessPiece {
    pub fn new(piece: Piece, color: Color) -> Self {
        Self { piece, color }
    }

    pub fn from_str(p: char) -> Option<ChessPiece> {
        match p {
            'r' => Some(ChessPiece::new(Piece::Rook, Color::Black)),
            'n' => Some(ChessPiece::new(Piece::Knight, Color::Black)),
            'b' => Some(ChessPiece::new(Piece::Bishop, Color::Black)),
            'q' => Some(ChessPiece::new(Piece::Queen, Color::Black)),
            'k' => Some(ChessPiece::new(Piece::King, Color::Black)),
            'p' => Some(ChessPiece::new(Piece::Pawn, Color::Black)),
            'R' => Some(ChessPiece::new(Piece::Rook, Color::White)),
            'N' => Some(ChessPiece::new(Piece::Knight, Color::White)),
            'B' => Some(ChessPiece::new(Piece::Bishop, Color::White)),
            'Q' => Some(ChessPiece::new(Piece::Queen, Color::White)),
            'K' => Some(ChessPiece::new(Piece::King, Color::White)),
            'P' => Some(ChessPiece::new(Piece::Pawn, Color::White)),
            _ => None,
        }
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

impl Piece {
    fn as_str(&self) -> &str {
        match self {
            Piece::Pawn => "p",
            Piece::Knight => "n",
            Piece::Bishop => "b",
            Piece::Rook => "r",
            Piece::Queen => "q",
            Piece::King => "k",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "w" => Self::White,
            "b" => Self::Black,
            _ => panic!("Invalid color option provided: {}", s),
        }
    }
}
