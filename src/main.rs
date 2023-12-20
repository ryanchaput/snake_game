// Making snake!
// Learning/inspo material: https://blog.scottlogic.com/2020/10/08/lets-build-snake-with-rust.html
// Pay attention to how the fns and files are sorted, best practices!

mod snake;
mod direction;
mod game;
mod point;
mod command;

use crate::game::Game;
use std::io::stdout;

/*
    TODO: Connect this to GitHub with ssh key!!!
    How to make this better?
    Current interface is pretty poor, hard to decipher the different columns
    Dimensions are very narrow and tall, not close to square
    Pop out window instead of playing in terminal?
    -   Add more color?
    -   Add obstacles? (other than just the snake)
    -   Increase speed? (easy)
 */

// Runs the game!
fn main() {
    Game::new(stdout(), 10, 10).run();
}
