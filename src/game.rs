use std::{
    collections::VecDeque,
    fmt::{self, Debug},
};

use crate::{errors::GameError, player::Player};

#[derive(Clone)]
pub struct Game {
    me: Player,
    opp: Player,
    queue: VecDeque<u8>,
}

impl Game {
    pub fn new(queue: VecDeque<u8>) -> Self {
        Game {
            me: Player::new(),
            opp: Player::new(),
            queue,
        }
    }

    pub fn play(&mut self, my_action: u8, opp_action: u8) -> Result<(), GameError> {
        let balls = self.queue.pop_front().unwrap();

        let me_result = self.me.play(balls, my_action);
        let opp_result = self.opp.play(balls, opp_action);

        match (me_result, opp_result) {
            (Ok(()), Ok(())) => (),
            (Err(_), Ok(())) => {
                self.me.reset_score();
                return Err(GameError::Lose);
            }
            (Ok(()), Err(_)) => {
                self.opp.reset_score();
                return Err(GameError::Win);
            }
            (Err(_), Err(_)) => {
                if self.me.get_score() > self.opp.get_score() {
                    return Err(GameError::Win);
                } else {
                    return Err(GameError::Lose);
                }
            }
        }

        let me_result = self.me.add_heads(self.opp.apply_nuisance());
        let opp_result = self.opp.add_heads(self.me.apply_nuisance());

        match (me_result, opp_result) {
            (Ok(()), Ok(())) => Ok(()),
            (Err(_), Ok(())) => {
                self.me.reset_score();
                Err(GameError::Lose)
            }
            (Ok(()), Err(_)) => {
                self.opp.reset_score();
                Err(GameError::Win)
            }
            (Err(_), Err(_)) => {
                if self.me.get_score() > self.opp.get_score() {
                    Err(GameError::Win)
                } else {
                    Err(GameError::Lose)
                }
            }
        }
    }

    pub fn add_balls(&mut self, new_balls: u8) {
        self.queue.push_back(new_balls);
    }

    pub fn get_me(&self) -> &Player {
        &self.me
    }

    pub fn get_opp(&self) -> &Player {
        &self.opp
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}   Points   {}",
            self.me.get_score(),
            self.opp.get_score()
        )?;
        writeln!(
            f,
            "{}  Nuisance  {}",
            self.me.get_nuisance(),
            self.opp.get_nuisance()
        )?;

        for i in 0..12 {
            let left = self.me.get_row(i);
            let right = self.opp.get_row(i);

            let top_balls = if i == 5 {
                let mut top_balls = String::new();
                for &ball in self.queue.iter() {
                    top_balls.push(char::from_digit(((ball >> 4) & 0xF) as u32, 10).unwrap_or('?'))
                }
                top_balls
            } else if i == 6 {
                let mut top_balls = String::new();
                for &ball in self.queue.iter() {
                    top_balls.push(char::from_digit((ball & 0xF) as u32, 10).unwrap_or('?'));
                }
                top_balls
            } else {
                String::from("        ")
            };

            writeln!(f, "{}  {}  {}", left, top_balls, right)?;
        }
        Ok(())
    }
}
