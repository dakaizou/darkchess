mod board;
mod piece;
mod rank;
mod cell;
mod game;

use game::Game;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Game>();
}
