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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisualizationKind {
    Execute,
    Write,
    Read,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Visualization {
    pub player: usize,
    pub kind: VisualizationKind,
}

pub struct Game {
    pub players: Players,
    pub memory: Vec<Instruction>,
    pub visualization: Vec<Option<Visualization>>,
    pub coresize: isize,
}

impl Game {
    pub fn new(num_players: usize, coresize: isize) -> Self {
        let mem_size = num_players * 5000;
        Game {
            players: Players {
                players: vec![],
                curr_player: 0,
            },
            visualization: vec![None; mem_size],
            memory: vec![dat_zero(); mem_size],
            coresize,
        }
    }

    pub fn debug_print_memory(&self) {
        let zero = dat_zero();

        for i in 0..self.memory.len() {
            if self.memory[i] != zero {
                println!("0x{:08x} : {}", i, self.memory[i]);
            }
        }
    }

    pub fn start_game(&mut self, codes: Vec<Vec<Instruction>>) {
        // initialize all player states, copy into memory space
        // add a process to prp

        let mut rng = rand::rng();

        let mut gap = (5000 * codes.len()) - codes.iter().fold(0, |acc, x| acc + x.len());
        let mut random_vals = vec![];
        for _ in 0..codes.len() {
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
                println!("{:?}", locations[positions[i]]);
                self.memory[locations[positions[i]] + j] = codes[i][j];
            }

            let new_player = UserState {
                processes: vec![locations[positions[i]]],
                curr_process: 0,
            };

            players.push(new_player);
        }
        self.players = Players {
            players,
            curr_player: 0,
        };
    }
}
