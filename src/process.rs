use crate::instruction::{dat_zero, Instruction, Op};
use rand::{seq::SliceRandom, thread_rng, Rng};

pub struct UserState {
    pub processes: Vec<usize>,
    pub curr_process: usize,
}

pub struct Players {
    pub players: Vec<UserState>,
    pub curr_player: usize,
}

impl Players {}

pub struct Game {
    pub players: Players,
    pub memory: Vec<Instruction>,
    pub coresize: isize,
}

impl Game {
    pub fn new(num_players: usize, coresize: isize) -> Self {
        Game {
            players: Players {
                players: vec![],
                curr_player: 0,
            },
            memory: vec![dat_zero(); num_players * 2000],
            coresize,
        }
    }

    pub fn start_game(&mut self, codes: Vec<Vec<Instruction>>) {
        // initialize all player states, copy into memory space
        // add a process to prp

        let mut rng = thread_rng();

        let mut gap = (2000 * codes.len()) - codes.iter().fold(0, |acc, x| acc + x.len());
        let mut random_vals = vec![];
        for i in 0..codes.len() {
            let g = rng.random_range(0..gap);
            gap -= g;
            random_vals.push(g);
        }

        *random_vals.last_mut().unwrap() += gap;

        let mut positions: Vec<usize> = (0..codes.len()).collect();
        positions.shuffle(&mut rng);

        let mut locations = vec![0];
        for i in 0..(codes.len() - 1) {
            let mut prev_pos = locations[locations.len() - 1];
            prev_pos += positions[i] + random_vals[i];
            locations.push(prev_pos);
        }

        let mut players = vec![];
        for i in 0..codes.len() {
            for j in 0..(codes[i].len()) {
                self.memory[locations[positions[i]] + j] = codes[i][j];
            }

            let new_player = UserState {
                processes: vec![locations[positions[i]]],
                curr_process: 0,
            };

            players.push(new_player);
        }
    }
}
