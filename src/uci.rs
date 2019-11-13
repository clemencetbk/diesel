use std::io::{self, Read, Write, stdin, stdout};

pub fn uci_loop() -> io::Result<()> {
    loop {
        let input_str = read();
        let input_slice: Vec<&str> = input_str.as_slice()
                               .iter()
                               .map(|x| x.as_str())
                               .collect(); 
        match input_slice.as_slice() {
            ["uci", ..] => { 
                println!("id\noption\nuciok\n");
                // TODO: add engine info and options
            }
            ["debug", "on"] => continue, // TODO: implement debug mode
            ["debug", "off"] => continue, // TODO: implement debug mode
            ["isready", ..] => println!("isready"),
            ["setoption", "name", "button", ..] => continue,
            ["setoption", "name", name, "value", value] => continue,
            ["register", "later", ..] => continue,
            ["register", "name", name, "code", code, ..] => continue,
            ["ucinewgame", ..] => continue,
            ["position", ..] => continue,
            // * position [fen  | startpos ]  moves  .... 
            ["go", ..] => continue,
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
            ["stop", ..] => continue,
            ["ponderhit", ..] => continue,
            ["quit", ..] => break,
            _ => continue,
        }
    }
    Ok(())
}   

pub fn read() -> Vec<String> {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: could not read input");
    let split = input.split_whitespace();
    split.map(String::from).collect()
}