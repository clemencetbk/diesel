// A collection of bitboards for each piece type/colour
pub struct Board {
    pub white_pawn: u64,
    pub white_rook: u64,
    pub white_knight: u64,
    pub white_bishop: u64,
    pub white_king: u64,
    pub white_queen: u64,
    
    pub black_pawn: u64,
    pub black_rook: u64,
    pub black_knight: u64,
    pub black_bishop: u64,
    pub black_king: u64,
    pub black_queen: u64
}

pub fn new() -> Board {       
    Board {
        white_pawn: (0b11111111 << 8),
        white_rook: 0b10000001,
        white_knight: 0b01000010,
        white_bishop: 0b00100100,
        white_king: 0b00010000,
        white_queen: 0b00001000,
        
        black_pawn: (0b11111111 << 50),
        black_rook: (0b10000001 << 58),
        black_knight: (0b01000010 << 58),
        black_bishop: (0b00100100 << 58), 
        black_king: (0b00010000 << 58),
        black_queen: (0b00001000 << 58),

    }
}