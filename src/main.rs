mod game;
mod player;
mod solver;

use std::collections::VecDeque;

use game::Game;
use rand::Rng;

fn get_piece() -> u8 {
    let mut rng = rand::thread_rng();
    let c1 = rng.gen_range(1..6);
    let c2 = rng.gen_range(1..6);
    (c1 << 4) | c2
}

fn main() {
    let base_color: VecDeque<u8> = (0..8).map(|_| get_piece()).collect();

    let mut game = Game::new(base_color);

    eprintln!("{:?}", game);

    game.play(0, 18);
    game.play(6, 18);
    game.play(5, 21);
    game.add_balls(get_piece());

    eprintln!("{:?}", game);
}
