use std::{
    collections::VecDeque,
    fmt::{self, Debug},
};

use crate::player::Player;

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

    pub fn play(&mut self, my_action: u8, opp_action: u8) {
        let balls = self.queue.pop_front().unwrap();
        self.me.play(balls, my_action);
        self.opp.play(balls, opp_action);

        let rows_opp = self.me.apply_nuisance();
        let rows_me = self.opp.apply_nuisance();

        self.me.add_heads(rows_me);
        self.opp.add_heads(rows_opp);
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
