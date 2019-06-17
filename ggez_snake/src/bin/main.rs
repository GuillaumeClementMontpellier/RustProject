// First we'll import the crates we need for our game;
// in this case that is just `ggez` and `rand`.
extern crate ggez;
extern crate rand;

// Bien sur, on utilise le jeu defini en librarie
use ggez_snake::SnakeGame;

fn main(){
  SnakeGame::run();
}
