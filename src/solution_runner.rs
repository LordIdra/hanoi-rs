use std::{mem::swap, collections::{HashSet, BTreeMap}};

use crate::{solution::Solution, problem::{Arrangement, get_initial_arrangement, is_final_arrangement}};

pub struct SolutionRunner {
    visited_hashes: BTreeMap<usize, HashSet<i32>>,
    arrangement: Arrangement,
    cycles: usize,
    program_counter: usize,
    tower_position: usize,
    held_value: Option<i32>,
    scratch: Option<i32>,
}

impl SolutionRunner {
    // Returns true if goal met
    pub fn run(solution: &Solution) -> bool {
        let arrangement = get_initial_arrangement();
        SolutionRunner {
            visited_hashes: BTreeMap::new(),
            arrangement, 
            cycles: 0, 
            program_counter: 0, 
            tower_position: 0,
            held_value: None,
            scratch: None,
        }.mainloop(solution)
    }

    fn mainloop(&mut self, solution: &Solution) -> bool {
        loop {
            let token = solution.get_token(self.program_counter);
            if token.is_none() {
                return false;
            }

            if token.unwrap() == ']' {
                let hash = self.hash_state();
                let visited_hashes = self.visited_hashes
                    .entry(self.program_counter)
                    .or_insert_with(HashSet::new);
                // Entered infinite loop
                if visited_hashes.contains(&hash) {
                    return false;
                }
                visited_hashes.insert(hash);
            }


            self.step(solution, token.unwrap());
            self.cycles += 1;
            if is_final_arrangement(&self.arrangement) {
                return true;
            }
        }
    }

    fn step(&mut self, solution: &Solution, token: char) {
        let mut next_program_counter = self.program_counter + 1;
        match token {
            '>' => { 
                self.tower_position += 1;
                if self.tower_position >= self.arrangement.len() {
                    self.tower_position -= self.arrangement.len();
                }
            }

            '<' => {
                if (self.tower_position as i32 - 1) < 0 {
                    self.tower_position += self.arrangement.len();
                }
                self.tower_position -= 1;
            }

            '.' => {
                let platform = &mut self.arrangement[self.tower_position];
                if self.held_value.is_none() {
                    if !platform.is_empty() {
                        self.held_value = platform.pop()
                    }
                } else if platform.is_empty() || platform[0] < self.held_value.unwrap() {
                    platform.push(self.held_value.unwrap());
                    self.held_value = None;
                }
            }

            '~' => {
                swap(&mut self.scratch, &mut self.held_value);
            }

            '[' => {
                if self.held_value.is_none() {
                    next_program_counter = solution.jump_forward(self.program_counter);
                }
            }

            ']' => {
                if self.held_value.is_some() {
                    next_program_counter = solution.jump_back(self.program_counter);
                }
            }

            _ => unreachable!()
        }

        self.program_counter = next_program_counter;
    }

    fn hash_state(&self) -> i32 {
        let mut hash = 0;

        hash +=      self.arrangement[0].len() as i32;
        hash += 4  * self.arrangement[1].len() as i32;
        hash += 16 * self.arrangement[2].len() as i32;
        hash += 64 * self.tower_position as i32;
        if let Some(scratch) = self.scratch {
            hash += 256 * scratch;
        }
        if let Some(held_value) = self.held_value {
            hash += 1024 * held_value;
        }

        hash
    }
}