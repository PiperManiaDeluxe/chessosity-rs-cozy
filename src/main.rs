use crate::uci::do_uci_loop;

mod perft;
mod uci;
mod eval;
mod eval_count_material;
mod eval_is_mate;
mod search;
mod time_manager;

fn main() {
    do_uci_loop();
}
