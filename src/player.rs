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

    pub fn apply_nuisance(&mut self) -> u32 {
        let rows = self.nuisance / 70;
        self.nuisance %= 70;
        rows
    }

    pub fn add_heads(&mut self, num_rows: u32) -> bool {
        if num_rows == 0 {
            return false;
        }

        for col in 0..6 {
            if self.bottom[col] < num_rows as usize {
                return true;
            }
        }
        for col in 0..6 {
            for _ in 0..num_rows {
                self.board[self.bottom[col]][col] = 0;
                self.bottom[col] -= 1;
            }
        }
        false
    }

    pub fn play(&mut self, balls: u8, position: u8) -> bool {
        let c1 = balls >> 4;
        let c2 = balls & 0xF;

        match position {
            0..6 => {
                // vertical 1 column
                let p = position as usize;
                let y = self.bottom[p];
                if y >= 2 {
                    self.board[y][p] = c2;
                    self.board[y - 1][p] = c1;
                    self.bottom[p] -= 2;
                } else if y == 1 {
                    self.board[y][p] = c2;
                    self.board[y - 1][p] = c1;
                    self.bottom[p] -= 1; // we cannot move the cursor to -1
                } else {
                    return false;
                }
            }
            6..12 => {
                // vertical 1 column - reversed
                let p = (position - 6) as usize;
                let y = self.bottom[p];
                if y >= 2 {
                    self.board[y][p] = c1;
                    self.board[y - 1][p] = c2;
                    self.bottom[p] -= 2;
                } else if y == 1 {
                    self.board[y][p] = c1;
                    self.board[y - 1][p] = c2;
                    self.bottom[p] -= 1; // we cannot move the cursor to -1
                } else {
                    return false;
                }
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

        let score = self.process_board();

        self.score += score;
        self.nuisance += score;

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

    fn process_board(&mut self) -> u32 {
        let mut score = 0u32;
        let mut changed = true;
        let mut cp = 0;
        while changed {
            let mut count_group = [0, 0, 0, 0, 0];
            let mut total = 0u32;
            changed = false;
            let groups = self.find_groups();
            if !groups.is_empty() {
                let mut gb = 0u32;
                for group in groups.iter() {
                    let (row, col) = group.first().unwrap();
                    let color = self.board[*row][*col] as usize;
                    count_group[color - 1] += 1;
                    total += group.len() as u32;
                    gb += match group.len() {
                        0..4 => 0,
                        4..10 => group.len() as u32 - 4,
                        _ => 8,
                    }
                }
                let cb = match count_group.iter().filter(|&x| *x > 0).count() {
                    1 => 0,
                    2 => 2,
                    3 => 4,
                    4 => 8,
                    5 => 16,
                    _ => 16,
                };
                score += (10 * total) * (cp + cb + gb);
                self.remove_groups(&groups);
                self.apply_gravity();
                changed = true;
            }
            cp = if cp == 0 { 8 } else { cp * 2 };
        }

        score
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
            for read in (1..12).rev() {
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
        let board: [[u8; 6]; 12] = [
            [7, 7, 7, 7, 7, 2],
            [7, 7, 7, 7, 7, 2],
            [7, 7, 7, 7, 7, 2],
            [7, 7, 7, 7, 7, 2],
            [7, 7, 7, 7, 7, 2],
            [7, 7, 7, 7, 7, 3],
            [7, 7, 7, 7, 7, 2],
            [7, 7, 4, 3, 7, 2],
            [7, 7, 4, 3, 7, 2],
            [7, 7, 3, 3, 7, 2],
            [7, 7, 4, 2, 7, 2],
            [7, 7, 4, 1, 7, 2],
        ];
        let bottom: [usize; 6] = [11, 11, 6, 6, 11, 11];
        let mut player = Player {
            board,
            score: 0,
            nuisance: 0,
            bottom,
        };

        let score = player.process_board();

        eprintln!("{:?}", player.board);

        assert!(player.board[11][2] == 7);
        assert!(score == 1070);
    }

    #[test]
    fn test_heads() {
        let board: [[u8; 6]; 12] = [
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 1, 4, 3, 7, 7],
            [7, 1, 4, 3, 7, 7],
            [7, 1, 3, 3, 7, 7],
            [7, 1, 4, 2, 7, 7],
            [7, 1, 4, 1, 7, 7],
        ];
        let bottom: [usize; 6] = [11, 6, 6, 6, 11, 11];
        let mut player = Player {
            board,
            score: 0,
            nuisance: 0,
            bottom,
        };

        let score = player.process_board();

        eprintln!("{:?}", player.board);

        assert!(player.board[11][2] == 7);
        eprintln!("{:?}", score);
        assert!(score == 270 + 320);
    }

    #[test]
    fn test_head() {
        let board: [[u8; 6]; 12] = [
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 7, 7, 7, 7, 7],
            [7, 1, 4, 3, 7, 7],
            [7, 1, 4, 3, 7, 7],
            [7, 1, 3, 3, 7, 7],
            [7, 1, 4, 2, 7, 7],
            [7, 1, 4, 1, 7, 7],
        ];
        let bottom: [usize; 6] = [11, 6, 6, 6, 11, 11];
        let mut player = Player {
            board,
            score: 0,
            nuisance: 0,
            bottom,
        };

        player.add_heads(3);

        eprintln!("{:?}", player.board);

        assert!(player.board[11][0] == 0);
        assert!(player.board[9][0] == 0);
        assert!(player.board[6][3] == 0);
        assert!(player.board[4][3] == 0);
        assert!(player.board[11][5] == 0);
        assert!(player.board[9][5] == 0);
    }
}
