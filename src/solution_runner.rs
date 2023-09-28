use std::mem::swap;

use crate::{solution::Solution, problem::{Problem, Arrangement}};

pub struct SolutionRunner {
    arrangement: Arrangement,
    cycles: usize,
    program_counter: usize,
    tower_position: usize,
    held_value: Option<i32>,
    scratch: Option<i32>,
}

impl SolutionRunner {
    // Returns true if goal met
    pub fn run(solution: &Solution, problem: Problem) -> bool {
        let arrangement = problem.get_initial_arrangement();
        SolutionRunner {
            arrangement, 
            cycles: 0, 
            program_counter: 0, 
            tower_position: 0,
            held_value: None,
            scratch: None,
        }.mainloop(solution, problem)
    }

    fn mainloop(&mut self, solution: &Solution, problem: Problem) -> bool {
        loop {
            // Prevent infinite loops
            if self.cycles > 500 {
                return false;
            }

            if self.step(solution) {
                //println!("Program finished without meeting a goal.");
                return false;
            }

            self.cycles += 1;

            if problem.is_final_arrangement(&self.arrangement) {
                //println!("Goal met!");
                return true;
            }
        }
    }

    // Returns true if program finished
    fn step(&mut self, solution: &Solution) -> bool {
        let token = solution.get_token(self.program_counter);
        if token.is_none() {
            return true;
        }

        let mut next_program_counter = self.program_counter + 1;
        match token.unwrap() {
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

        false
    }
}