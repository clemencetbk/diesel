extern crate diesel;

#[cfg(test)]
pub use diesel::board::*;

#[test]
fn test_fen_empty() {
    let mut b = Board::new();
    let init = String::from("8/8/8/8/8/8/8/8 w KQkq - 0 1");
    from_fen(&init, &mut b);
    for i in 0..14 {
        assert_eq!(b.bitboards[i], 0);
    }
}

#[test]
fn test_fen_few() {
    let mut b = Board::new();
    let init = String::from("r7/8/8/8/8/8/8/1N3Q2 w KQkq - 0 1");
    from_fen(&init, &mut b);
    for i in 0..14 {
        if (i != ROOK | BLACK) && (i != KNIGHT | WHITE) && (i != QUEEN | WHITE) {
            assert_eq!(b.bitboards[i as usize], 0);
        }
    }
    assert_eq!(get_bitboard(&b, ROOK, BLACK), 1);
    assert_eq!(get_bitboard(&b, KNIGHT, WHITE), 1 << 57);
    assert_eq!(get_bitboard(&b, QUEEN, WHITE), 1 << 61);
}

// Helper function for indexing into board's bitboards
fn get_bitboard(board: &Board, piecetype: u8, colour: u8) -> u64 {
    board.bitboards[(piecetype | colour) as usize]
}