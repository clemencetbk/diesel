mod board;
mod uci;
mod test;

fn main() {
    match uci::uci_loop() {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}
