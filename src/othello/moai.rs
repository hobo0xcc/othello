use rand::{prelude, Rng};
use std::collections::{HashMap, VecDeque};
use std::io;
use std::ops::{Shl, Shr};
use wasm_timer::Instant;
// use std::time::{Duration, Instant};

fn vertical_mirror(x: i64) -> i64 {
    x.swap_bytes()
}

#[derive(Clone, Copy)]
pub struct BitBoard {
    black: u64,
    white: u64,
    player: i32,
}

impl BitBoard {
    pub fn new(player: i32) -> Self {
        Self {
            black: 0,
            white: 0,
            player,
        }
    }

    pub fn from_strings(board: Vec<String>, player: i32) -> Self {
        let mut black = 0;
        let mut white = 0;
        for (y, column) in board.iter().enumerate() {
            for (x, ch) in column.chars().enumerate() {
                if ch == '0' {
                    black |= 0x8000_0000_0000_0000 >> (y * 8 + x);
                } else if ch == '1' {
                    white |= 0x8000_0000_0000_0000 >> (y * 8 + x);
                }
            }
        }

        Self {
            black,
            white,
            player,
        }
    }

    pub fn legal_move_bits(&self, P: u64, O: u64) -> u64 {
        let mut moves: u64;
        let mut mO: u64;
        let mut flip1: u64;
        let mut flip7: u64;
        let mut flip9: u64;
        let mut flip8: u64;
        let mut pre1: u64;
        let mut pre7: u64;
        let mut pre9: u64;
        let mut pre8: u64;

        mO = O & 0x7e7e7e7e7e7e7e7e;

        flip1 = mO & (P << 1);
        flip7 = mO & (P << 7);
        flip9 = mO & (P << 9);
        flip8 = O & (P << 8);
        flip1 |= mO & (flip1 << 1);
        flip7 |= mO & (flip7 << 7);
        flip9 |= mO & (flip9 << 9);
        flip8 |= O & (flip8 << 8);
        pre1 = mO & (mO << 1);
        pre7 = mO & (mO << 7);
        pre9 = mO & (mO << 9);
        pre8 = O & (O << 8);
        flip1 |= pre1 & (flip1 << 2);
        flip7 |= pre7 & (flip7 << 14);
        flip9 |= pre9 & (flip9 << 18);
        flip8 |= pre8 & (flip8 << 16);
        flip1 |= pre1 & (flip1 << 2);
        flip7 |= pre7 & (flip7 << 14);
        flip9 |= pre9 & (flip9 << 18);
        flip8 |= pre8 & (flip8 << 16);
        moves = flip1 << 1;
        moves |= flip7 << 7;
        moves |= flip9 << 9;
        moves |= flip8 << 8;
        flip1 = mO & (P >> 1);
        flip7 = mO & (P >> 7);
        flip9 = mO & (P >> 9);
        flip8 = O & (P >> 8);
        flip1 |= mO & (flip1 >> 1);
        flip7 |= mO & (flip7 >> 7);
        flip9 |= mO & (flip9 >> 9);
        flip8 |= O & (flip8 >> 8);
        pre1 >>= 1;
        pre7 >>= 7;
        pre9 >>= 9;
        pre8 >>= 8;
        flip1 |= pre1 & (flip1 >> 2);
        flip7 |= pre7 & (flip7 >> 14);
        flip9 |= pre9 & (flip9 >> 18);
        flip8 |= pre8 & (flip8 >> 16);
        flip1 |= pre1 & (flip1 >> 2);
        flip7 |= pre7 & (flip7 >> 14);
        flip9 |= pre9 & (flip9 >> 18);
        flip8 |= pre8 & (flip8 >> 16);
        moves |= flip1 >> 1;
        moves |= flip7 >> 7;
        moves |= flip9 >> 9;
        moves |= flip8 >> 8;

        return moves & !(P | O);
    }

    // pub fn legal_move_bits(&self, p: u64, o: u64) -> u64 {
    //     let mut moves: i64;
    //     let mut mO: i64;
    //     let mut flip1: i64;
    //     let mut pre1: i64;
    //     let mut flip8: i64;
    //     let mut pre8: i64;
    //     let mut pp: __m128i;
    //     let mut mOO: __m128i;
    //     let mut mm: __m128i;
    //     let mut flip: __m128i;
    //     let mut pre: __m128i;

    //     let (p, o) = (p as i64, o as i64);

    //     unsafe {
    //         mO = o & 0x7e7e_7e7e_7e7e_7e7e;
    //         pp = _mm_set_epi64x(vertical_mirror(p), p);
    //         mOO = _mm_set_epi64x(vertical_mirror(mO), mO);

    //         flip = _mm_and_si128(mOO, _mm_slli_epi64(pp, 7));
    //         flip1 = mO & (p << 1);
    //         flip8 = o & (p << 8);

    //         flip = _mm_or_si128(flip, _mm_and_si128(mOO, _mm_slli_epi64(flip, 7)));
    //         flip1 |= mO & (flip1 << 1);
    //         flip8 |= o & (flip8 << 8);

    //         pre = _mm_and_si128(mOO, _mm_slli_epi64(mOO, 7));
    //         pre1 = mO & (mO << 1);
    //         pre8 = o & (o << 8);

    //         flip = _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 14)));
    //         flip1 |= pre1 & (flip1 << 2);
    //         flip8 |= pre8 & (flip8 << 16);

    //         flip = _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 14)));
    //         flip1 |= pre1 & (flip1 << 2);
    //         flip8 |= pre8 & (flip8 << 16);

    //         mm = _mm_slli_epi64(flip, 7);
    //         moves = flip1 << 1;
    //         moves |= flip8 << 8;

    //         flip = _mm_and_si128(mOO, _mm_slli_epi64(pp, 9));
    //         flip1 = mO & (p >> 1);
    //         flip8 = o & (p >> 8);

    //         flip = _mm_or_si128(flip, _mm_and_si128(mOO, _mm_slli_epi64(flip, 9)));
    //         flip1 |= mO & (flip1 >> 1);
    //         flip8 |= o & (p >> 8);

    //         pre = _mm_and_si128(mOO, _mm_slli_epi64(mOO, 9));
    //         pre1 >>= 1;
    //         pre8 >>= 8;

    //         flip = _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 18)));
    //         flip1 |= pre1 & (flip1 >> 2);
    //         flip8 |= pre8 & (flip8 >> 16);

    //         flip = _mm_or_si128(flip, _mm_and_si128(pre, _mm_slli_epi64(flip, 18)));
    //         flip1 |= pre1 & (flip1 >> 2);
    //         flip8 |= pre8 & (flip8 >> 16);

    //         mm = _mm_or_si128(mm, _mm_slli_epi64(flip, 9));
    //         moves |= flip1 >> 1;
    //         moves |= flip8 >> 8;

    //         moves |= _mm_cvtsi128_si64(mm)
    //             | vertical_mirror(_mm_cvtsi128_si64(_mm_unpackhi_epi64(mm, mm)));
    //     }

    //     return (moves & !(p | o)) as u64;
    // }

    pub fn legal_moves(&self, p: u64, o: u64) -> Vec<u64> {
        let legal_move_bits = self.legal_move_bits(p, o);
        let mut legals = Vec::new();
        for i in 0..64 {
            if legal_move_bits & (1 << i) != 0 {
                legals.push(legal_move_bits & (1 << i));
            }
        }

        legals
    }

    pub fn curr_board(&self) -> (u64, u64) {
        match self.player {
            0 => (self.black, self.white),
            1 => (self.white, self.black),
            _ => panic!(),
        }
    }

    pub fn transfer(&self, position: u64, k: i32) -> u64 {
        match k {
            0 => (position << 8) & 0xffffffffffffff00,
            1 => (position << 7) & 0x7f7f7f7f7f7f7f00,
            2 => (position >> 1) & 0x7f7f7f7f7f7f7f7f,
            3 => (position >> 9) & 0x007f7f7f7f7f7f7f,
            4 => (position >> 8) & 0x00ffffffffffffff,
            5 => (position >> 7) & 0x00fefefefefefefe,
            6 => (position << 1) & 0xfefefefefefefefe,
            7 => (position << 9) & 0xfefefefefefefe00,
            _ => 0,
        }
    }

    pub fn update(&self, player: u64, opponent: u64, position: u64) -> (u64, u64) {
        let mut rev: u64 = 0;
        for k in 0..8 {
            let mut rev_: u64 = 0;
            let mut mask: u64 = self.transfer(position, k);
            while mask != 0 && mask & opponent != 0 {
                rev_ |= mask;
                mask = self.transfer(mask, k);
            }

            if mask & player != 0 {
                rev |= rev_;
            }
        }

        let player = player ^ (position | rev);
        let opponent = opponent ^ rev;

        (player, opponent)
    }

    pub fn is_game_ended(&self) -> bool {
        let white = self.legal_move_bits(self.white, self.black);
        let black = self.legal_move_bits(self.black, self.white);

        white == 0 && black == 0
    }

    pub fn winner(&self) -> i32 {
        unsafe {
            let white_cnt = self.white.count_ones();
            let black_cnt = self.black.count_ones();
            if black_cnt > white_cnt {
                return 0;
            } else if white_cnt > black_cnt {
                return 1;
            } else {
                return 2;
            }
        }
    }

    pub fn show_state(&self) {
        let mut count = 1;
        for i in (0..64).rev() {
            match ((self.white & (1 << i)) >> i, ((self.black) & (1 << i)) >> i) {
                (1, 0) => eprint!("1"),
                (0, 1) => eprint!("0"),
                (0, 0) => eprint!("."),
                _ => panic!(),
            }
            if count % 8 == 0 {
                eprintln!();
            }
            count += 1;
        }
    }

    fn next_player(&self) -> i32 {
        1 - self.player
    }

    pub fn play(&self, position: u64) -> Self {
        let mut board = self.clone();
        let (player, opponent) = match self.player {
            0 => (board.black, board.white),
            1 => (board.white, board.black),
            _ => panic!(),
        };
        let (mut player, mut opponent) = board.update(player, opponent, position);
        match board.player {
            0 => {
                board.black = player;
                board.white = opponent;
            }
            1 => {
                board.white = player;
                board.black = opponent;
            }
            _ => panic!(),
        }
        board.player = board.next_player();
        // if board.legal_move_bits(opponent, player) == 0 {
        //     board.player = board.next_player();
        // }

        board
    }

    pub fn playout(&self) -> f64 {
        let mut rng = rand::thread_rng();
        let mut board = self.clone();
        unsafe {
            while !board.is_game_ended() {
                let (player, opponent) = match board.player {
                    0 => (board.black, board.white),
                    1 => (board.white, board.black),
                    _ => panic!(),
                };
                let legal_moves = board.legal_moves(player, opponent);
                if legal_moves.len() == 0 {
                    board.player = board.next_player();
                    continue;
                }

                let index = rng.gen_range(0..legal_moves.len());
                let legal = legal_moves[index];
                let (player, opponent) = board.update(player, opponent, legal);
                match board.player {
                    0 => {
                        board.black = player;
                        board.white = opponent;
                    }
                    1 => {
                        board.white = player;
                        board.black = opponent;
                    }
                    _ => panic!(),
                }
                board.player = board.next_player();
            }
        }

        let winner = board.winner();
        if winner == 2 {
            0.0
        } else if winner == self.player {
            1.0
        } else {
            -1.0
        }
    }
}

type NodeId = usize;

pub struct MCTS {
    table: HashMap<NodeId, Node>,
    curr_id: NodeId,
    cp: f64,
    playout: i32,
}

impl MCTS {
    pub fn new(cp: f64, playout: i32) -> Self {
        Self {
            table: HashMap::new(),
            curr_id: 0,
            cp,
            playout,
        }
    }

    fn gen_id(&mut self) -> NodeId {
        let id = self.curr_id;
        self.curr_id += 1;
        id
    }

    pub fn show(&self, id: NodeId) {
        let state = self.table.get(&id).unwrap().state.clone();
        eprintln!("{} {}", state.player, self.table.get(&id).unwrap().q);
        if !self.table.get(&id).unwrap().children.is_empty() {
            self.show(*self.table.get(&id).unwrap().children.get(0).unwrap());
        }
    }

    pub fn run(&mut self, state: BitBoard, time: u128) -> (u64, i32) {
        let mut v_0 = Node::new(None, state, 0);
        let root_id = self.gen_id();
        self.table.insert(root_id, v_0);
        let mut inst = Instant::now();
        let mut count = 0;
        loop {
            let v_l = self.tree_policy(root_id);
            let reward = self.default_policy(v_l);
            self.backup(v_l, -reward);
            count += 1;
            if count % 50 == 0 {
                if inst.elapsed().as_millis() >= time {
                    break;
                }
            }
        }

        let best_child = self.best_child_ucb_tuned(root_id, 0.0);
        let best_child = self.table.get(&best_child).unwrap();
        (best_child.action, count)
    }

    fn tree_policy(&mut self, mut id: NodeId) -> NodeId {
        let mut v = self.table.get(&id).unwrap();
        let mut v_id = id;
        while !v.state.is_game_ended() {
            if v.is_not_fully_expanded() {
                return self.expand(v_id);
            } else {
                if v.no_legal_moves() && v.children.len() == 0 {
                    let mut node = v.clone();
                    node.state.player = node.state.next_player();
                    node.parent = Some(v_id);
                    let (player, opponent) = node.state.curr_board();
                    node.untried = node
                        .state
                        .legal_moves(player, opponent)
                        .into_iter()
                        .collect();
                    let node_id = self.gen_id();
                    self.table.insert(node_id, node);
                    self.table.get_mut(&v_id).unwrap().children.push(node_id);
                    return node_id;
                } else {
                    v_id = self.best_child_ucb_tuned(v_id, self.cp);
                    v = self.table.get(&v_id).unwrap();
                }
            }
        }

        v_id
    }

    fn expand(&mut self, id: NodeId) -> NodeId {
        let new_id = self.gen_id();
        let mut v = self.table.get_mut(&id).unwrap();
        let a = v.untried.pop_back().unwrap();
        let new_state = v.state.play(a);
        let mut vp = Node::new(Some(id), new_state, a);
        v.children.push(new_id);
        drop(v);
        self.table.insert(new_id, vp);

        new_id
    }

    fn best_child_ucb_tuned(&self, id: NodeId, c: f64) -> NodeId {
        let v = self.table.get(&id).unwrap();
        let mut max = f64::NEG_INFINITY;
        let mut res_id = 0;
        for child_id in v.children.iter() {
            let child = self.table.get(child_id).unwrap();
            let mut val = (child.q / child.n as f64);
            let v_i = child.var + f64::sqrt(2.0 * f64::log2(v.n as f64) / child.n as f64);
            val += f64::sqrt(f64::log2(v.n as f64) / child.n as f64 * f64::min(1.0 / 4.0, v_i));
            if val > max {
                max = val;
                res_id = *child_id;
            }
        }

        res_id
    }

    fn best_child(&self, id: NodeId, c: f64) -> NodeId {
        let v = self.table.get(&id).unwrap();
        let mut max = f64::NEG_INFINITY;
        let mut res_id = 0;
        for child_id in v.children.iter() {
            let child = self.table.get(child_id).unwrap();
            let mut val = (child.q / child.n as f64)
                + c * f64::sqrt(2.0 * f64::log2(v.n as f64) / child.n as f64);
            if val > max {
                max = val;
                res_id = *child_id;
            }
        }

        res_id
    }

    fn default_policy(&self, v: NodeId) -> f64 {
        let mut reward = 0.0;
        for i in 0..self.playout {
            reward += self.table.get(&v).unwrap().state.playout()
        }

        reward
    }

    fn backup(&mut self, v: NodeId, mut reward: f64) {
        let mut v = Some(v);
        while let Some(id) = v {
            let mut node = self.table.get_mut(&id).unwrap();
            let ave = node.q / node.n as f64;
            node.n += 1;
            node.q += reward;
            node.square =
                (node.square * (node.n as f64 - 1.0) + f64::powi(reward, 2)) / node.n as f64;
            node.var = node.square - f64::powi(node.q / node.n as f64, 2);
            reward = -reward;
            v = node.parent;
        }
    }
}

#[derive(Clone)]
struct Node {
    pub parent: Option<NodeId>,
    pub state: BitBoard,
    pub action: u64,
    pub untried: VecDeque<u64>,
    pub children: Vec<NodeId>,
    pub n: usize,
    pub q: f64,
    pub square: f64,
    pub var: f64,
}

impl Node {
    pub fn new(parent: Option<NodeId>, state: BitBoard, action: u64) -> Self {
        let (player, opponent) = match state.player {
            0 => (state.black, state.white),
            1 => (state.white, state.black),
            _ => panic!(),
        };
        let untried: VecDeque<u64> = state.legal_moves(player, opponent).into_iter().collect();
        Self {
            parent,
            state,
            action,
            untried,
            children: Vec::new(),
            n: 0,
            q: 0.0,
            square: 0.0,
            var: 0.0,
        }
    }

    pub fn is_not_fully_expanded(&self) -> bool {
        self.untried.len() != 0
    }

    pub fn no_legal_moves(&self) -> bool {
        let (player, opponent) = self.state.curr_board();

        self.state.legal_move_bits(player, opponent) == 0
    }
}
