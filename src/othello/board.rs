#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Disk {
    White,
    Black,
    Empty,
}

impl Disk {
    pub fn opponent(&self) -> Disk {
        match *self {
            Disk::Black => Disk::White,
            Disk::White => Disk::Black,
            Disk::Empty => Disk::Empty,
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[derive(Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub disks: Vec<Vec<Disk>>,
    pub player: i32,
}

impl Default for Board {
    fn default() -> Self {
        let mut disks = Vec::new();
        for y in 0..8 {
            let mut column = Vec::new();
            for x in 0..8 {
                column.push(Disk::Empty);
            }

            disks.push(column);
        }

        disks[3][3] = Disk::White;
        disks[3][4] = Disk::Black;
        disks[4][3] = Disk::Black;
        disks[4][4] = Disk::White;

        Self {
            width: 8,
            height: 8,
            disks,
            player: 0,
        }
    }
}

pub fn coordinate(pos: &str) -> (usize, usize) {
    let ch = pos.chars().nth(0).unwrap() as u8;
    let x = ch - 'a' as u8;
    let ch = pos.chars().nth(1).unwrap() as u8;
    let y = ch - '1' as u8;

    (x as usize, y as usize)
}

impl Board {
    pub fn new(width: usize, height: usize, player: i32) -> Self {
        let mut disks = Vec::new();
        for i in 0..height {
            let mut column = Vec::new();
            for j in 0..width {
                column.push(Disk::Empty);
            }
            disks.push(column);
        }
        Self {
            width,
            height,
            disks,
            player,
        }
    }

    pub fn from_strings(board: Vec<String>, player: i32) -> Self {
        let height = board.len();
        let width = board[0].len();

        let mut disks = Vec::new();
        for y in 0..height {
            let mut column = Vec::new();
            let chars: Vec<char> = board[y].chars().collect();
            for x in 0..width {
                if chars[x] == '.' {
                    column.push(Disk::Empty);
                } else if chars[x] == '0' {
                    column.push(Disk::Black);
                } else if chars[x] == '1' {
                    column.push(Disk::White);
                } else {
                    panic!("unknown char: {}", chars[x]);
                }
            }
            disks.push(column);
        }

        Self {
            width,
            height,
            disks,
            player,
        }
    }

    pub fn player_disk(&self) -> Disk {
        if self.player == 0 {
            Disk::Black
        } else if self.player == 1 {
            Disk::White
        } else {
            panic!("unknown player: {}", self.player);
        }
    }

    pub fn next_player(&self) -> i32 {
        1 - self.player
    }

    fn update_adjacent_disks(
        &mut self,
        disk: Disk,
        x: isize,
        y: isize,
        inc_x: isize,
        inc_y: isize,
    ) -> bool {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            return false;
        }
        if self.disks[y as usize][x as usize] == disk {
            return true;
        } else if self.disks[y as usize][x as usize] == Disk::Empty {
            return false;
        }
        if self.update_adjacent_disks(disk, x + inc_x, y + inc_y, inc_x, inc_y) {
            self.disks[y as usize][x as usize] = disk;
            true
        } else {
            false
        }
    }

    fn update(&mut self, disk: Disk, x: usize, y: usize) {
        self.disks[y][x] = disk;
        for inc_y in -1..=1 {
            for inc_x in -1..=1 {
                if inc_x == 0 && inc_y == 0 {
                    continue;
                }

                self.update_adjacent_disks(
                    disk,
                    x as isize + inc_x,
                    y as isize + inc_y,
                    inc_x,
                    inc_y,
                );
            }
        }
    }

    pub fn is_game_ended(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.disks[y][x] == Disk::Empty {
                    return false;
                }
            }
        }

        return true;
    }

    fn can_turn_over(
        &self,
        disk: Disk,
        x: isize,
        y: isize,
        inc_x: isize,
        inc_y: isize,
        count: i32,
    ) -> i32 {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            return 0;
        }
        if self.disks[y as usize][x as usize] == disk {
            return count;
        } else if self.disks[y as usize][x as usize] == disk.opponent() {
            return self.can_turn_over(disk, x + inc_x, y + inc_y, inc_x, inc_y, count + 1);
        } else {
            return 0;
        }
    }

    pub fn is_legal_move(&self, disk: Disk, x: usize, y: usize) -> bool {
        if self.disks[y][x] != Disk::Empty {
            return false;
        }
        let mut sum = 0;
        for inc_y in -1..=1 {
            for inc_x in -1..=1 {
                if inc_x == 0 && inc_y == 0 {
                    continue;
                }

                sum += self.can_turn_over(
                    disk,
                    x as isize + inc_x,
                    y as isize + inc_y,
                    inc_x,
                    inc_y,
                    0,
                );
            }
        }

        sum > 0
    }

    pub fn legal_moves(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_legal_move(self.player_disk(), x, y) {
                    res.push((x, y));
                }
            }
        }

        res
    }

    pub fn play(&self, x: usize, y: usize) -> Board {
        let mut next_board = self.clone();
        next_board.update(self.player_disk(), x, y);
        next_board.player = next_board.next_player();

        next_board
    }

    pub fn num_disk(&self) -> (i32, i32) {
        let mut white = 0;
        let mut black = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                match self.disks[y][x] {
                    Disk::White => white += 1,
                    Disk::Black => black += 1,
                    _ => {}
                }
            }
        }

        (black, white)
    }

    fn winner(&self) -> i32 {
        let (black, white) = self.num_disk();

        // println!("white: {}, black: {}", white, black);

        if black > white {
            0
        } else if white > black {
            1
        } else {
            2
        }
    }

    pub fn to_strings(&self) -> Vec<String> {
        let mut res = Vec::new();
        for y in 0..self.height {
            let mut s = String::new();
            for x in 0..self.width {
                if self.disks[y][x] == Disk::Black {
                    s.push('0');
                } else if self.disks[y][x] == Disk::White {
                    s.push('1');
                } else {
                    s.push('.');
                }
            }
            res.push(s);
        }

        res
    }
}