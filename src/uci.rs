use std::io::{self, Read, Write, stdin, stdout};
use std::io::BufRead;

pub fn uci_loop() -> io::Result<()> {
    loop {
        let command = read_to_space();
        match command.as_ref() {
            "uci" => { 
                println!("id\noption\nuciok\n");
                // TODO: add engine info and options
            }
            "debug" => continue, // TODO: implement debug mode
            "isready" => println!("isready"),
            "setoption" => setoption(),
            "register" => continue,
            "ucinewgame" => continue,
            "position" => continue,
            // * position [fen  | startpos ]  moves  .... 
            "go" => continue,
	        // * searchmoves  .... 
	        // * ponder
	        // * wtime 
	        // * btime 
	        // * winc 
	        // * binc 
	        // * movestogo 
	        // * depth 
	        // * nodes 
	        // * mate 
	        // * movetime 
	        // * infinite
            "stop" => continue,
            "ponderhit" => continue,
            "quit" => break,
            _ => continue,
        }
    }
    Ok(())
}   

pub fn setoption() {
    // Read tokens from stdin. Handle cases:
    // ["setoption", "name", name, "value", value]
    // ["setoption", "name", "button"] -- no value needed
}

pub fn read() -> Vec<String> {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: could not read input");
    let split = input.split_whitespace();
    split.map(String::from).collect()
}

// Only read input until first whitespace
pub fn read_to_space() -> String {
    let mut buf = vec![];
    stdin().lock()
        .read_until(b' ', &mut buf)
        .expect("Error: could not read input");
    String::from_utf8_lossy(&buf).to_string()
}