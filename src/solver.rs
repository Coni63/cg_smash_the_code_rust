// https://www.codingame.com/blog/smash-code-contest-report/?utm_source=codingame&utm_medium=details-page&utm_campaign=cg-blog&utm_content=stc
// https://www.codingame.com/blog/stochastic-algorithm-smash-the-code/?utm_source=codingame&utm_medium=details-page&utm_campaign=cg-blog&utm_content=stc

use std::time::Instant;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{errors::GameError, game::Game, get_piece};

#[derive(Debug)]
pub struct Score {
    pub max_diff: i32,
    pub sum_score: i32,
    pub num_win: i32,
    pub num_lose: i32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            max_diff: 0,
            sum_score: 0,
            num_win: 0,
            num_lose: 0,
        }
    }
}

pub fn decode(position: u8) -> (u8, u8) {
    match position {
        0..6 => (position, 0),
        6..12 => (position - 6, 2),
        12..17 => (position - 12, 1),
        17..22 => (position - 17, 3),
        _ => panic!("What have you done with positions !"),
    }
}

pub fn solve(game: &Game) -> (u8, u8) {
    let start = Instant::now();
    let mut rng: StdRng = StdRng::seed_from_u64(222);

    let mut vec_score: Vec<Score> = (0..22).map(|_| Score::new()).collect();

    for _ in 0..2000 {
        let mut copy = game.clone();

        let mut new = true;
        let mut first_action = 0;
        loop {
            let my_action = rng.gen_range(0..22);
            let opp_action = rng.gen_range(0..22);

            if new {
                first_action = my_action as usize;
                new = false;
            }

            let state = copy.play(my_action, opp_action);
            copy.add_balls(get_piece(&mut rng));

            match state {
                Ok(()) => (),
                Err(GameError::Win) => {
                    let delta = copy.get_me().get_score() as i32 - copy.get_me().get_score() as i32;
                    vec_score[first_action].num_win += 1;
                    vec_score[first_action].sum_score += copy.get_me().get_score() as i32;
                    vec_score[first_action].max_diff =
                        std::cmp::max(delta, vec_score[first_action].max_diff);
                    break;
                }
                Err(GameError::Lose) => {
                    let delta = copy.get_me().get_score() as i32 - copy.get_me().get_score() as i32;
                    vec_score[first_action].num_lose += 1;
                    vec_score[first_action].sum_score += copy.get_me().get_score() as i32;
                    vec_score[first_action].max_diff =
                        std::cmp::max(delta, vec_score[first_action].max_diff);
                    break;
                }
                Err(GameError::BoardIsFull) => panic!("This should not be reached"),
            }
        }

        // eprintln!(
        //     "{} vs {}",
        //     copy.get_me().get_score(),
        //     copy.get_opp().get_score()
        // );

        // if (copy.get_me().get_score() > 0) || (copy.get_opp().get_score() > 0) {
        //     eprintln!("{:?}", copy,);
        //     // break;
        // }
    }

    let duration = start.elapsed();
    eprintln!("Time elapsed in expensive_function() is: {:?}", duration);

    eprintln!("{:?}", vec_score);

    let mut best_ratio = 0f32;
    let mut best_move = 0;
    for (first_move, score) in vec_score.iter().enumerate() {
        let ratio = score.num_win as f32 / score.num_lose as f32;
        if ratio > best_ratio {
            best_ratio = ratio;
            best_move = first_move;
        }
        eprintln!("{} -> {} vs {}", first_move, score.num_win, score.num_lose);
    }
    eprintln!("Picker : {}", best_move);

    decode(best_move as u8)
}
