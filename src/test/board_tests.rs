#[cfg(test)]
use super::super::board::*;

#[test]
fn test() { // TODO: write tests
    let b = Board::new();
    assert_eq!(b.castling_rights, 0);
}