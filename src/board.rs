#![allow(dead_code)]
// Index definitions
pub const WHITE: u8 = 0;
pub const BLACK: u8 = 1;

pub const PAWN: u8 = 2;
pub const ROOK: u8 = 4;
pub const KNIGHT: u8 = 6;
pub const BISHOP: u8 = 8;
pub const KING: u8 = 10;
pub const QUEEN: u8 = 12;

// For castling rights
pub const WHITE_KING: u8 = 1;
pub const WHITE_QUEEN: u8 = 2;
pub const BLACK_KING: u8 = 4;
pub const BLACK_QUEEN: u8 = 8;

pub struct Board {
    pub bitboards: [u64; 14],
    pub castling_rights: u8,
    pub en_passant: u8,
    pub turn: u8,
    pub half_moves: u8
}

impl Board {
    pub fn new() -> Board {      
        let bitboards: [u64; 14] = [0; 14];
        let mut board = Board {
            bitboards: bitboards,
            castling_rights: 0,
            en_passant: 0,
            turn: 0,
            half_moves: 0
        };
        let init = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        from_fen(&init, &mut board);
        board
    }
}

// Accept string in FEN format modify board to store information from the string.
// [board information] [current turn] [castling rights] [en passant] [halfmove clock] [fullmove clock]
// See https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
pub fn from_fen(fen: &String, board: &mut Board) {
  // lower case: black; upper case: white
    board.bitboards = [0; 14]; // re-init bitboards 
    let fen_str = fen.to_string();
    let mut tokens = fen_str.split_whitespace();

    let mut get_next = || -> &str {tokens.next().unwrap()};

    let board_info = get_next();
    let turn = get_next();
    let castling_rights = get_next();
    let en_passant = get_next();
    let half_moves = get_next();
    // skip fullmove clock as we won't use it

    set_board(board_info, board);
    set_turn(turn, board);
    set_castling_rights(castling_rights, board);
    set_en_passant(en_passant, board);
    set_half_moves(half_moves, board);
}


// Helper functions to parse information

fn set_board(board_str: &str, board: &mut Board) {
    let mut piecetype;
    let mut col;
    let mut shift = 0;
    println!("{}", board_str);
    for (_, &curr_byte) in board_str.as_bytes().iter().enumerate() {
        let curr_char = curr_byte as char;
        if curr_char.is_digit(10) {
            shift += curr_char.to_digit(10).unwrap();
        } else {
            match curr_char.to_lowercase().to_string().as_ref() {
                "p" => piecetype = PAWN,
                "r" => piecetype = ROOK,
                "n" => piecetype = KNIGHT,
                "b" => piecetype = BISHOP,
                "q" => piecetype = QUEEN,
                "k" => piecetype = KING,
                _ => continue
            }
            if curr_char.is_lowercase() {
                col = BLACK;
            } else {
                col = WHITE;
            }
            board.bitboards[(col | piecetype) as usize] = 1 << shift;
            shift += 1; 
        }
    }
}

fn set_turn(turn: &str, board: &mut Board) {
    match turn.to_lowercase().to_string().as_ref() {
        "w" => board.turn = WHITE,
        "b" => board.turn = BLACK,
        _ => ()
    }
}

fn set_castling_rights(castling_rights: &str, board: &mut Board) {
    board.castling_rights = 0;
    for &item in castling_rights.as_bytes().iter() {
        match (item as char).to_string().as_ref() {
            "k" => board.castling_rights |= BLACK_KING,
            "q" => board.castling_rights |= BLACK_QUEEN,
            "K" => board.castling_rights |= WHITE_KING,
            "Q" => board.castling_rights |= WHITE_QUEEN,
            _ => ()
        }
    }
}

fn set_en_passant(en_passant: &str, board: &mut Board) {
    // en passant will either "-" or a letter followed by a number 1-8, like "e6"
    // en passant square is stored with a number: enumerate all positions as: a1 -> 1, ... a8 -> 8, b1 -> 9, b2 -> 10, ...
    // the letter (col) indicates how many times 8
    // the number (row) indicates how much to add for the remainder
    if en_passant == "-" {
        board.en_passant = 0;
        return
    }
    let mut col = 0;
    match en_passant.chars().nth(0).unwrap() {
        'a' => col = 0,
        'b' => col = 1,
        'c' => col = 2,
        'd' => col = 3,
        'e' => col = 4,
        'f' => col = 5,
        'g' => col = 6,
        'h' => col = 7,
        _ => ()
    }

    let row = en_passant.chars().nth(1).unwrap().to_digit(10).unwrap() as u8;

    board.en_passant = col * 8 + row;
}

fn set_half_moves(half_moves: &str, board: &mut Board) {
    board.half_moves = half_moves.parse::<u8>().unwrap();
}