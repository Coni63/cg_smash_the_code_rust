mod errors;
mod game;
mod player;
mod solver;

use std::collections::VecDeque;

use game::Game;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

fn get_piece(rng: &mut StdRng) -> u8 {
    let c1 = rng.gen_range(1..6);
    let c2 = rng.gen_range(1..6);
    (c1 << 4) | c2
}

fn main() {
    let mut rng: StdRng = StdRng::seed_from_u64(222);
    let base_color: VecDeque<u8> = (0..8).map(|_| get_piece(&mut rng)).collect();

    let mut game = Game::new(base_color);

    eprintln!("{:?}", game);

    // game.play(0, 18);
    // game.add_balls(get_piece(&mut rng));
    // game.play(6, 18);
    // game.add_balls(get_piece(&mut rng));
    // game.play(5, 21);
    // game.add_balls(get_piece(&mut rng));

    let (column, rotation) = solver::solve(&game);

    println!("{} {}", column, rotation);
}
