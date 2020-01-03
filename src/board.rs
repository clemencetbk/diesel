#![allow(dead_code)]
// Index definitions
pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const PAWN: usize = 2;
pub const ROOK: usize = 4;
pub const KNIGHT: usize = 6;
pub const BISHOP: usize = 8;
pub const KING: usize = 10;
pub const QUEEN: usize = 12;

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

fn init_bitboards(mut bitboards: [u64; 14]) {
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

pub fn from_fen(fen: &String, board: &mut Board) {
  // lower case: black; upper case: white
    board.bitboards = [0; 14]; // re-init bitboards 
    let mut shift = 0;
    let mut col;
    let mut piecetype = PAWN;
    let bytes = fen.as_bytes();
    for (_, &item) in bytes.iter().enumerate() {
        if item == b'/' {
            continue;
        } else if item == b' ' {
            break;
            // TODO: Handle additional info: castling rights, en passant, half move, full move
        } else if (item as char).is_digit(10) {
            shift += (item as char).to_digit(10).unwrap();
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
            } else {
                col = WHITE;
            }
            board.bitboards[col | piecetype] = 1 << shift;
            shift += 1; 
        }
    }
}