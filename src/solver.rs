// https://www.codingame.com/blog/smash-code-contest-report/?utm_source=codingame&utm_medium=details-page&utm_campaign=cg-blog&utm_content=stc
// https://www.codingame.com/blog/stochastic-algorithm-smash-the-code/?utm_source=codingame&utm_medium=details-page&utm_campaign=cg-blog&utm_content=stc

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{errors::GameError, game::Game, get_piece};

pub fn solve(game: &Game) -> [u8; 8] {
    let mut rng: StdRng = StdRng::seed_from_u64(222);

    for _ in 0..1000 {
        let mut copy = game.clone();

        loop {
            let my_action = rng.gen_range(0..22);
            let opp_action = rng.gen_range(0..22);

            match copy.play(my_action, opp_action) {
                Ok(()) => copy.add_balls(get_piece(&mut rng)),
                Err(GameError::Win) => break,
                Err(GameError::Lose) => break,
                Err(GameError::BoardIsFull) => panic!("This should not be reached"),
            }
        }

        eprintln!(
            "{} vs {}",
            copy.get_me().get_score(),
            copy.get_opp().get_score()
        );

        if (copy.get_me().get_score() == 0) & (copy.get_opp().get_score() == 0) {
            eprintln!("{:?}", copy,);
            break;
        }
    }

    [0; 8]
}
