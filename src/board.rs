#![allow(dead_code)]
// Index definitions
const WHITE: usize = 0;
const BLACK: usize = 1;

const PAWN: usize = 2;
const ROOK: usize = 4;
const KNIGHT: usize = 6;
const BISHOP: usize = 8;
const KING: usize = 10;
const QUEEN: usize = 12;

pub struct Board {
    pub bitboards: [u64; 12],
    pub castling_rights: u8,
    pub en_passant: u8
}

impl Board {
    pub fn new() -> Board {      
        let bitboards: [u64; 12] = [0; 12];
        init_bitboards(bitboards);
        Board {
            bitboards: bitboards,
            castling_rights: 0,
            en_passant: 0
        }
    }
}

fn init_bitboards(mut bitboards: [u64; 12]) {
    for col in 0..2 {
        for i in 1..7 {
            let piecetype = 2 * i;
            let mut shift = 0;
            let mut pawns = 8;
            if col == BLACK {
                shift = 58;
                pawns = -8;
            }
            let mut b: u64 = 0;
            match piecetype {
                PAWN => b = 0b11111111 << (shift + pawns),
                ROOK => b = 0b10000001 << shift,
                KNIGHT => b = 0b01000010 << shift,
                BISHOP => b = 0b00100100 << shift,
                KING => b =  0b00010000 << shift,
                QUEEN => b = 0b00001000 << shift,
                _ => ()
            }
            bitboards[col | piecetype] = b;
        }
    }
}

fn from_fen(fen: &String, board: &mut Board) {
  // FEN example:  rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
  // lower case: black; upper case: white
    let mut shift_file = 0;
    let mut shift_rank = 0;
    let mut col = WHITE;
    let mut piecetype = PAWN;
    let bytes = fen.as_bytes();
    for (_, &item) in bytes.iter().enumerate() {
        if item == b'/' {
            shift_file += 8;
            //return &fen[0..i];
        } else if item == b' ' {

        } else {
            match (item as char).to_lowercase().to_string().as_ref() {
                "r" => piecetype = ROOK,
                "n" => piecetype = KNIGHT,
                "b" => piecetype = BISHOP,
                "q" => piecetype = QUEEN,
                "k" => piecetype = KING,
                _ => ()
            }
            if (item as char).is_lowercase() {
                col = BLACK;
            }
            shift_rank += 1;
        }
        board.bitboards[col | piecetype] = 1 << shift_rank + shift_file;
    }
}