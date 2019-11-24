use std::io::{self, stdin, Error};
use std::io::BufRead;

pub fn uci_loop() -> io::Result<()> {
    loop {
        let mut buf = String::new();
        match stdin().lock().read_line(&mut buf) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
        let mut buf_bytes = buf.as_bytes();
        match read_arg(&mut buf_bytes).unwrap().as_ref() {
            "uci" => { 
                println!("id\noption\nuciok\n");
                // TODO: add engine info and options
            }
            "debug" => continue, // TODO: implement debug mode
            "isready" => println!("isready\n"),
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
            "stop" => break,
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

pub fn read_arg(input: &mut dyn BufRead) -> Result<String, Error> {
    let mut buf = vec![];
    match input.read_until(b' ', &mut buf) {
        Err(e) => return Err(e),
        _ => return Ok(String::from_utf8_lossy(&buf).to_string())
    }
}