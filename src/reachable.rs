use crate::single_puzzle::SinglePuzzle;
use crate::scramble::Scramble;
use crate::helpers::{get_all_moves};
use std::fs::{File, create_dir_all};
use std::io::{BufWriter, BufReader};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Batch {
    pub batch_size: usize,
    pub states: Vec<SinglePuzzle>,
}

impl Batch {
    pub fn new(batch_size: usize) -> Self {
        Batch {
            batch_size,
            states: Vec::with_capacity(batch_size),
        }
    }

    pub fn is_full(&self) -> bool {
        self.states.len() >= self.batch_size
    }

    pub fn add_state(&mut self, state: SinglePuzzle) {
        self.states.push(state);
    }

    pub fn sort_states(&mut self) {
        self.states.sort();
    }

    pub fn save_to_file(&self, path: &str) {
        let file = File::create(path).expect("Failed to create batch file");
        let mut writer = BufWriter::new(file);
        for puzzle in &self.states {
            puzzle.save_binary_to_file(&mut writer);
        }
    }

    pub fn load_from_file(path: &str, with_opposite_move: bool) -> Self {
        let file = File::open(path).expect("Failed to open batch file");
        let mut reader = BufReader::new(file);
        let mut states = Vec::new();
        while let Some(puzzle) = crate::single_puzzle::SinglePuzzle::load_binary_from_file(&mut reader, with_opposite_move) {
            states.push(puzzle);
        }
        Batch {
            batch_size: states.len(),
            states,
        }
    }
}

pub struct ReachableStates {
    pub depth: usize,
    pub batch_size: usize,
    pub batch_files: Vec<String>,
    pub store_directory: String,
    pub with_opposite_move: bool,
}

impl ReachableStates {
    pub fn new(
        depth: usize,
        puzzle: SinglePuzzle,
        batch_size: usize,
        store_directory: String,
        with_opposite_move: bool,
        num_score_weakens: usize,
        improve: bool,
    ) -> Self {
        create_dir_all(&store_directory).expect("Failed to create store directory");
        let batch_files = Vec::new();
        let mut batch = Batch::new(batch_size);
        let mut batch_count = 0;
        let mut reachable_states = Self {
            depth,
            batch_size,
            batch_files,
            store_directory: store_directory.clone(),
            with_opposite_move,
        };
        reachable_states.compute_reachable(
            depth,
            &get_all_moves(),
            Scramble { moves: Vec::new() },
            vec![puzzle.calculate_score()],
            num_score_weakens,
            improve,
            puzzle,
            &mut batch,
            &mut batch_count,
        );
        if !batch.states.is_empty() {
            batch.sort_states();
            let batch_path = format!("{}/batch_{}.bin", store_directory, batch_count);
            batch.save_to_file(&batch_path);
            reachable_states.batch_files.push(batch_path);
            reachable_states.sort_batches();
        }
        reachable_states
    }

    pub fn print_first_5(&self, with_opposite_move: bool) {
        let mut count = 0;
        for batch_path in &self.batch_files {
            let batch = Batch::load_from_file(batch_path, with_opposite_move);
            for state in &batch.states {
                println!("{:?}", state);
                count += 1;
                if count >= 5 {
                    return;
                }
            }
        }
    }

    pub fn sort_batches(&mut self) {
        for j in 0..self.batch_files.len().saturating_sub(1) {
            let i = self.batch_files.len() - 2 - j;
            let batch_path_a = &self.batch_files[i];
            let batch_a = Batch::load_from_file(batch_path_a, self.with_opposite_move);
            let batch_path_b = &self.batch_files[i + 1];
            let batch_b = Batch::load_from_file(batch_path_b, self.with_opposite_move);
            let states_a = batch_a.states;
            let states_b = batch_b.states;
            let mut merged_states = Vec::new();
            merged_states.extend(states_a);
            merged_states.extend(states_b);
            merged_states.sort();
            let batch = Batch {
                states: merged_states[0..batch_a.batch_size].to_vec(),
                batch_size: batch_a.batch_size,
            };
            batch.save_to_file(&batch_path_a);
            let batch = Batch {
                states: merged_states[batch_a.batch_size..].to_vec(),
                batch_size: batch_b.batch_size,
            };
            batch.save_to_file(&batch_path_b);
        }
    }

    pub fn compute_reachable(
        &mut self,
        depth: usize,
        all_moves: &Vec<crate::single_puzzle::Move>,
        scramble: Scramble,
        scores: Vec<i64>,
        num_score_weakens: usize,
        improve: bool,
        puzzle: SinglePuzzle,
        batch: &mut Batch,
        batch_count: &mut usize,
    ) {
        let mut current_puzzle = puzzle.clone();
        current_puzzle.apply_scramble(scramble.clone());
        if improve {
            if scores.len() > num_score_weakens {
                let current_score = current_puzzle.calculate_score();
                let critical_score = scores[scores.len() - 1 - num_score_weakens];
                if current_score < critical_score {
                    return;
                }
            }
        } else {
            if scores.len() > num_score_weakens {
                let current_score = current_puzzle.calculate_score();
                let critical_score = scores[scores.len() - 1 - num_score_weakens];
                if current_score > critical_score {
                    return;
                }
            }
        }
        for (i, mv) in all_moves.iter().enumerate() {
            if depth == self.depth {
                let progress = (i as f64 / all_moves.len() as f64) * 100.0;
                println!("Progress: {:.2}%", progress);
            }
            if depth == 0 {
                let mut cloned_puzzle = puzzle.clone();
                let mut new_scramble = scramble.clone();
                new_scramble.moves.push(mv.clone());
                cloned_puzzle.apply_scramble(new_scramble.clone());

                batch.add_state(cloned_puzzle);
                if batch.is_full() {
                    batch.sort_states();
                    let batch_path = format!("{}/batch_{}.bin", self.store_directory, *batch_count);
                    batch.save_to_file(&batch_path);
                    self.batch_files.push(batch_path);
                    self.sort_batches();
                    *batch = Batch::new(self.batch_size);
                    *batch_count += 1;
                }
            } else {
                let mut new_scramble = scramble.clone();
                new_scramble.moves.push(mv.clone());
                let mut new_puzzle = puzzle.clone();
                new_puzzle.apply_scramble(new_scramble.clone());
                let mut new_scores = scores.clone();
                new_scores.push(new_puzzle.calculate_score());
                self.compute_reachable(
                    depth - 1,
                    all_moves,
                    new_scramble,
                    new_scores,
                    num_score_weakens,
                    improve,
                    puzzle.clone(),
                    batch,
                    batch_count,
                );
            }
        }
    }

    pub fn overlaps(&self, other: &Self, with_opposite_move: bool) -> Option<Scramble> {
        let mut i_batch = 0;
        let mut j_batch = 0;
        let mut i = 0;
        let mut j = 0;
        while i_batch < self.batch_files.len() && j_batch < other.batch_files.len() {
            let batch_a_path = &self.batch_files[i_batch];
            let batch_a = Batch::load_from_file(batch_a_path, with_opposite_move);
            let batch_b_path = &other.batch_files[j_batch];
            let batch_b = Batch::load_from_file(batch_b_path, with_opposite_move);
            while i < batch_a.states.len() && j < batch_b.states.len() {
                match batch_a.states[i].cmp(&batch_b.states[j]) {
                    std::cmp::Ordering::Equal => {
                        let first_part_of_scramble = batch_a.states[i].get_scramble();
                        let second_part_of_scramble = batch_b.states[j].get_scramble().invert();
                        return Some(first_part_of_scramble.concat(second_part_of_scramble));
                    }
                    std::cmp::Ordering::Less => i += 1,
                    std::cmp::Ordering::Greater => j += 1,
                }
            }
            if i == batch_a.states.len() {
                i_batch += 1;
                i = 0;
            }
            if j == batch_b.states.len() {
                j_batch += 1;
                j = 0;
            }
        }
        None
    }
}