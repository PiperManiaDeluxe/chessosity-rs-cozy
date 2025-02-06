use crate::uci::uci_loop::do_uci_loop;

mod uci;
mod perft;
mod search;
mod eval;

fn main() {
    do_uci_loop();
}