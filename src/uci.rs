use std::io::{self, Read, Write, stdin, stdout, ErrorKind};
use std::io::BufRead;
use std::error::Error;
use memchr;

pub fn uci_loop() -> io::Result<()> {
    loop {
        let mut buf = String::new();
        stdin().lock().read_line(&mut buf);
        let mut buf_bytes = buf.as_bytes();
        match read_arg(&mut buf_bytes).as_ref() {
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
//    read_to_space();
    // Read tokens from stdin. Handle cases:
    // ["setoption", "name", name, "value", value]
    // ["setoption", "name", "button"] -- no value needed
}

pub fn read_arg(input: &mut BufRead) -> String {
    let mut buf = vec![];
    input.read_until(b' ', &mut buf);
    String::from_utf8_lossy(&buf).to_string()
}

pub fn read() -> Vec<String> {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: could not read input");
    let split = input.split_whitespace();
    split.map(String::from).collect()
}