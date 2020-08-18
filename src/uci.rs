use std::io::{self, stdin, Error};
use std::io::BufRead;
use diesel::board::*;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;


macro_rules! nth {
    ($args: expr, $pos: expr) => {
        $args.nth($pos).unwrap_or_default()
    };
}

pub fn uci_loop() -> io::Result<()> {
    let mut board = Board::new();
    let searching = Arc::new(AtomicUsize::new(1));
    loop {
        let args_str = read_args()?;
        let mut args = args_str.split_whitespace();
        match nth!(args, 0) {
            "uci" => send_info(),
            "debug" => continue, // TODO: implement debug mode
            "isready" => println!("readyok"),
            "setoption" => setoption(),
            "register" => continue,
            "ucinewgame" => board = Board::new(),
            "position" => set_position(args, &mut board),
            "go" => {
                println!("Searching...");
                let searching_clone = searching.clone();
                thread::spawn(move || {
                    while searching_clone.load(Ordering::SeqCst) != 0 {
                        search();
                    }
                    println!("Stopping search")
                });
            }
            "stop" => searching.store(0, Ordering::SeqCst), // terminate search
            "ponderhit" => continue,
            "quit" => break,
            x => println!("{}", x)
        }
    }
    Ok(())
}  

pub fn set_position(args: std::str::SplitWhitespace, board: &mut Board) {
    let mut fen = String::new();
    let tokens = args.skip(1);
    for token in tokens {
        match token {
            "moves" => {
                from_fen(&fen, board);
                break
            },
            _ => fen = fen + " " + token,
        }
    }
}

pub fn read_args() -> Result<String, Error> {
    let mut buf_str = String::new();
    stdin().lock().read_line(&mut buf_str)?;
    let res = buf_str.to_string();
    Ok(res)
}

pub fn send_info() {
    println!("id name diesel");
    println!("id author Clémence");
    // TODO: add options
    println!("uciok");
}

pub fn setoption() {
    // Read tokens from stdin. Handle cases:
    // ["setoption", "name", name, "value", value]
    // ["setoption", "name", "button"] -- no value needed
}

// TODO
fn search() {

}