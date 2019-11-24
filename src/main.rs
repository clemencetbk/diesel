#![feature(slice_patterns)]
mod board;
mod uci;

fn main() {
    match uci::uci_loop() {
        Err(e) => println!("{}", e),
        _ => ()
    }
}
