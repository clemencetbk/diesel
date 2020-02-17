#[cfg(test)]
use super::super::board::*;

#[test]
fn test() { // TODO: write tests
    let b = Board::new();
    assert_eq!(b.castling_rights, 0);
}

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
            assert_eq!(b.bitboards[i], 0);
        }
    }
    assert_eq!(b.bitboards[ROOK | BLACK], 1);
    assert_eq!(b.bitboards[KNIGHT | WHITE], 1 << 57);
    assert_eq!(b.bitboards[QUEEN | WHITE], 1 << 61);
}