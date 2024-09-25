// https://www.codingame.com/blog/smash-code-contest-report/?utm_source=codingame&utm_medium=details-page&utm_campaign=cg-blog&utm_content=stc
// https://www.codingame.com/blog/stochastic-algorithm-smash-the-code/?utm_source=codingame&utm_medium=details-page&utm_campaign=cg-blog&utm_content=stc

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{game::Game, get_piece};

pub fn solve(game: &Game) -> [u8; 8] {
    let mut rng: StdRng = StdRng::seed_from_u64(222);

    for _ in 0..1000 {
        let mut copy = game.clone();

        for _ in 0..20 {
            let my_action = rng.gen_range(0..22);
            let opp_action = rng.gen_range(0..22);
            let game_over = copy.play(my_action, opp_action);
            if game_over {
                break;
            } else {
                copy.add_balls(get_piece(&mut rng));
            }
        }

        eprintln!(
            "{} vs {}",
            copy.get_me().get_score(),
            copy.get_opp().get_score()
        )
    }

    [0; 8]
}
