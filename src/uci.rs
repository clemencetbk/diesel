use std::io::{self, Read, stdin, stdout};

pub fn uci_loop() -> io::Result<()> {
    // Tokens to handle:
    // uci
    // debug
    // isready
    // setoption
    // register 
    // ucinewgame
    // position
    // go
    // stop 
    // ponderhit
    // quit
    Ok(())
}   

pub fn get_input() -> Vec<String> {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: could not read input");
    let split = input.split_whitespace();
    split.map(String::from).collect()
}