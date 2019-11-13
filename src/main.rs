#![feature(slice_patterns)]
mod board;
mod uci;

fn main() {
    uci::uci_loop();
}
