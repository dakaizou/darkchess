mod board;
mod cell;
mod game;
mod piece;
mod rank;

use game::Game;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Game>::new().render();
}
