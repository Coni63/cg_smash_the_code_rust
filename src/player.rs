use rand::Error;

#[derive(Clone)]
pub struct Player {
    board: [[u8; 6]; 12],
    score: u32,
    nuisance: u32,
    bottom: [usize; 6],
}

impl Player {
    pub fn new() -> Self {
        Player {
            board: [[7; 6]; 12],
            score: 0,
            nuisance: 0,
            bottom: [11; 6],
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_nuisance(&self) -> u32 {
        self.nuisance
    }

    pub fn play(&mut self, balls: u8, position: u8) -> bool {
        let c1 = balls >> 4;
        let c2 = balls & 0xF;

        match position {
            0..6 => {
                // vertical 1 column
                let p = position as usize;
                let y = self.bottom[p];
                if y < 1 {
                    return false;
                }
                self.board[y][p] = c2;
                self.board[y - 1][p] = c1;
                self.bottom[p] -= 2;
                true
            }
            6..12 => {
                // vertical 1 column - reversed
                let p = (position - 6) as usize;
                let y = self.bottom[p];
                if y < 1 {
                    return false;
                }
                self.board[y][p] = c1;
                self.board[y - 1][p] = c2;
                self.bottom[p] -= 2;
                true
            }
            12..17 => {
                // horizontal
                let p = (position - 12) as usize;
                let y = self.bottom[p];
                let y2 = self.bottom[p + 1];
                if y == 0 || y2 == 0 {
                    return false;
                }
                self.board[y][p] = c1;
                self.board[y2][p + 1] = c2;
                self.bottom[p] -= 1;
                self.bottom[p + 1] -= 1;
                true
            }
            17..22 => {
                // horizontal
                let p = (position - 17) as usize;
                let y = self.bottom[p];
                let y2 = self.bottom[p + 1];
                if y == 0 || y2 == 0 {
                    return false;
                }
                self.board[y][p] = c2;
                self.board[y2][p + 1] = c1;
                self.bottom[p] -= 1;
                self.bottom[p + 1] -= 1;
                true
            }
            _ => panic!("What have you done with positions !"),
        }
    }

    pub fn get_row(&self, row: usize) -> String {
        let mut ans = String::new();
        for &value in self.board[row].iter() {
            ans.push(if value == 7 {
                '-'
            } else {
                char::from_digit(value as u32, 10).unwrap_or('?')
            });
        }
        ans
    }
}
