use std::array;
use std::fmt;
use std::fmt::Display;

use crate::pieces::types::ChessPiece;
use crate::pieces::types::Color;

pub struct GameState {
    board: Board,
    side_to_move: Color,
    castling_ability: String,
    en_passant_target_square: String,
    halfmove_clock: usize,
    fullmove_counter: usize,
}

impl Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.board.display();
        write!(
            f,
            "Side to Move: {:?}\nCastling Ability: {:?}\nEnPassant Target: {:?}\nHalfmove Clock: {:?}\nFullmove Counter: {:?}",
            self.side_to_move,
            self.castling_ability,
            self.en_passant_target_square,
            self.halfmove_clock,
            self.fullmove_counter
        )
    }
}

impl GameState {
    pub fn new() -> Self {
        let new_game_fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Self::from_fen_string(new_game_fen)
    }

    pub fn from_fen_string(fen_str: &str) -> Self {
        let fen_sections: Vec<&str> = fen_str.split(" ").collect();
        assert_eq!(fen_sections.len(), 6);

        let piece_placement = fen_sections[0];
        let board: Board = Board::from_fen_string(piece_placement);

        let side_to_move = Color::from_str(fen_sections[1]);
        let castling_ability = fen_sections[2].to_string();
        let en_passant_target_square = fen_sections[3].to_string();
        let halfmove_clock = usize::from_str_radix(fen_sections[4], 10).unwrap();
        let fullmove_counter = usize::from_str_radix(fen_sections[5], 10).unwrap();

        Self {
            board,
            side_to_move,
            castling_ability,
            en_passant_target_square,
            halfmove_clock,
            fullmove_counter,
        }
    }
}

pub struct Board {
    board: [Square; 64],
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

impl Square {
    fn new(occupied: Option<ChessPiece>) -> Self {
        Self { occupied }
    }
}

impl Board {
    fn init() -> Self {
        Self {
            board: array::from_fn(|_| Square { occupied: None }),
        }
    }

    ///https://www.chessprogramming.org/Forsyth-Edwards_Notation
    ///
    /// Starting FEN String
    ///
    /// `Piece Placement` `Color to Move` `Enpassant Target Square` `Castling Ability` `Halftime Clock` `Fullmove Counter`
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    ///
    /// Starts from the top left square and moves across the files
    /// to the next rank.
    ///
    /// Numbers denote empty spaces
    ///
    /// Upper Case = White, Lower Case = Black
    pub fn from_fen_string(fen_str: &str) -> Self {
        let fen_str: Vec<&str> = fen_str.split(" ").collect();

        let piece_placement = fen_str[0];

        let mut board: Board = Board::init();

        let mut idx: usize = 0;
        for row in piece_placement.split("/") {
            let characters: Vec<char> = row.chars().collect();
            // println!("{:?}", characters);
            for e in characters {
                if e.is_alphabetic() {
                    board.board[idx] = Square::new(ChessPiece::from_str(e));
                    idx += 1;
                } else {
                    let empty_count: u32 = e.to_digit(10).unwrap();
                    for _ in 0..empty_count {
                        board.board[idx] = Square::new(None);
                        idx += 1;
                    }
                }
            }
        }

        board
    }

    fn display(&self) {
        let mut rank: usize = 0;
        println!(" ----------------------------- ");
        loop {
            if rank >= 8 {
                break;
            }

            let id0 = rank * 8;
            let idf = (rank + 1) * 8;

            // println!("Rank: {rank}\nStart Index: {id0}\nEnd Index: {idf}");

            let row: &[Square] = &self.board[id0..idf];

            let display: String = row
                .iter()
                .map(|p| format!(" {} ", p))
                .collect::<Vec<String>>()
                .join("|");

            println!("{display}");

            rank += 1;
        }
        println!(" ----------------------------- ");
    }
}

mod test {
    use super::*;
    use crate::pieces::types::{Color, Piece};

    #[test]
    #[ignore = ""]
    fn test_display() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_fen_string(fen_str);
        board.display();

        let fen_str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let board = Board::from_fen_string(fen_str);
        board.display();
    }

    #[test]
    fn build_game() {
        let gs = GameState::new();
        // gs.board.display();
        println!("{}", gs);

        let fen_str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let gs = GameState::from_fen_string(fen_str);
        // gs.board.display();
        println!("{}", gs);
    }
}
