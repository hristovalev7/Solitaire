#![allow(non_snake_case)]

mod assets;
mod card;
mod game;
use crate::game::game::Game;

fn main() {
    Game::start();
}
