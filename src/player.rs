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
            }
            _ => panic!("What have you done with positions !"),
        }

        self.process_board();

        true
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

    fn process_board(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            let groups = self.find_groups();
            if !groups.is_empty() {
                self.remove_groups(&groups);
                self.apply_gravity();
                changed = true;
            }
        }
    }

    fn find_groups(&mut self) -> Vec<Vec<(usize, usize)>> {
        let mut groups = Vec::new();
        let mut visited = [[false; 6]; 12];

        for row in 0..12 {
            for col in 0..6 {
                if !visited[row][col] && self.board[row][col] > 0 && self.board[row][col] < 6 {
                    let mut group = Vec::new();
                    self.dfs(row, col, self.board[row][col], &mut visited, &mut group);
                    if group.len() >= 4 {
                        groups.push(group);
                    }
                }
            }
        }

        groups
    }

    fn dfs(
        &mut self,
        row: usize,
        col: usize,
        color: u8,
        visited: &mut [[bool; 6]; 12],
        group: &mut Vec<(usize, usize)>,
    ) {
        if row >= 12 || col >= 6 || visited[row][col] || self.board[row][col] != color {
            return;
        }

        visited[row][col] = true;
        group.push((row, col));

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;
            if new_row >= 0 && new_col >= 0 {
                self.dfs(new_row as usize, new_col as usize, color, visited, group);
            }
        }
    }

    fn remove_groups(&mut self, groups: &[Vec<(usize, usize)>]) {
        for group in groups {
            for &(row, col) in group {
                self.board[row][col] = 7;
            }
        }
    }

    fn apply_gravity(&mut self) {
        for col in 0..6 {
            let mut write = 11;
            for read in (0..12).rev() {
                if self.board[read][col] != 7 {
                    self.board[write][col] = self.board[read][col];
                    if write != read {
                        self.board[read][col] = 7;
                    }
                    write -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity() {
        let mut board: [[u8; 6]; 12] = [
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 4, 3, 7, 7],
            [7, 7, 4, 3, 7, 7],
            [7, 7, 3, 3, 7, 7],
            [7, 7, 4, 2, 7, 7],
            [7, 7, 4, 1, 7, 7],
        ];
        let bottom: [usize; 6] = [11, 11, 11, 11, 11, 11];
        let mut player = Player {
            board,
            score: 0,
            nuisance: 0,
            bottom,
        };

        player.process_board();

        eprintln!("{:?}", player.board);

        assert!(player.board[11][2] == 7);
    }
}
