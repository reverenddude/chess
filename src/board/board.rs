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
        write!(
            f,
            "Side to Move: {:?}\nCastling Ability: {:?}\nEnPassant Target: {:?}\nHalfmove Clock: {:?}\nFullmove Counter: {:?}\n\n{}",
            self.side_to_move,
            self.castling_ability,
            self.en_passant_target_square,
            self.halfmove_clock,
            self.fullmove_counter,
            self.board,
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
        let board: Board = Board::from_fen_str(piece_placement);

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
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,

    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "White Pawns:\n{:#066b}
White Knights:\n{:#066b}
White Bishops:\n{:#066b}
White Rooks:\n{:#066b}
White Queens:\n{:#066b}
White Kings:\n{:#066b}
Black Pawns:\n{:#066b}
Black Knights:\n{:#066b}
Black Bishops:\n{:#066b}
Black Rooks:\n{:#066b}
Black Queens:\n{:#066b}
Black Kings:\n{:#066b}
            ",
            self.white_pawns,
            self.white_knights,
            self.white_bishops,
            self.white_rooks,
            self.white_queens,
            self.white_king,
            self.black_pawns,
            self.black_knights,
            self.black_bishops,
            self.black_rooks,
            self.black_queens,
            self.black_king
        )
    }
}

impl Board {
    fn init() -> Self {
        Self {
            white_pawns: 0u64,
            white_knights: 0u64,
            white_bishops: 0u64,
            white_rooks: 0u64,
            white_queens: 0u64,
            white_king: 0u64,
            black_pawns: 0u64,
            black_knights: 0u64,
            black_bishops: 0u64,
            black_rooks: 0u64,
            black_queens: 0u64,
            black_king: 0u64,
        }
    }

    fn from_fen_str(fen_str: &str) -> Self {
        let mut board = Self::init();

        let fen_str: Vec<&str> = fen_str.split(" ").collect();

        let piece_placement = fen_str[0];

        let mut idx: usize = 0;
        for row in piece_placement.split("/") {
            let characters: Vec<char> = row.chars().collect();
            // println!("{:?}", characters);
            for e in characters {
                match e {
                    'P' => board.white_pawns |= 1u64 << idx,
                    'N' => board.white_knights |= 1u64 << idx,
                    'B' => board.white_bishops |= 1u64 << idx,
                    'R' => board.white_rooks |= 1u64 << idx,
                    'Q' => board.white_queens |= 1u64 << idx,
                    'K' => board.white_king |= 1u64 << idx,
                    'p' => board.black_pawns |= 1u64 << idx,
                    'n' => board.black_knights |= 1u64 << idx,
                    'b' => board.black_bishops |= 1u64 << idx,
                    'r' => board.black_rooks |= 1u64 << idx,
                    'q' => board.black_queens |= 1u64 << idx,
                    'k' => board.black_king |= 1u64 << idx,
                    _ => {
                        idx += e.to_digit(10).unwrap() as usize;
                        continue;
                    }
                }
                idx += 1;
            }
        }
        board
    }
}

mod test {
    use super::*;

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
