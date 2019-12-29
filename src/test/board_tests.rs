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
        println!("{}, {}", i, format!("{:b}", b.bitboards[i]));
        assert_eq!(b.bitboards[i], 0);
    }
}
