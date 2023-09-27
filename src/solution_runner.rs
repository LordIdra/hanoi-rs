use crate::{solution::Solution, problem::{Problem, Arrangement}};

pub struct SolutionRunner {
    arrangement: Arrangement,
    problem: Problem,
    solution: Solution,
    cycles: usize,
    program_counter: usize,
    tower_position: usize,
    held_value: Option<i32>,
}

impl SolutionRunner {
    pub fn run(solution: Solution, problem: Problem) {
        SolutionRunner {
            arrangement: problem.get_initial_arrangement(), 
            problem,
            solution, 
            cycles: 0, 
            program_counter: 0, 
            tower_position: 0,
            held_value: None,
        }.mainloop();
    }

    fn mainloop(&mut self) {
        loop {
            if self.step() {
                println!("Program finished without meeting a goal.");
                break;
            }

            self.cycles += 1;
    
            if self.problem.is_final_arrangement(&self.arrangement) {
                println!("Goal met!");
                break;
            }
        }
    }

    // Returns true if program finished
    fn step(&mut self) -> bool {
        let token = self.solution.get_token(self.program_counter-1);
        if token.is_none() {
            return true;
        }

        let mut next_program_counter = self.program_counter + 1;
        match token.unwrap() {
            '>' => self.tower_position += 1,
            '<' => self.tower_position -= 1,
            '.' => {
                let platform = &mut self.arrangement[self.tower_position];
                if self.held_value.is_none() && !platform.is_empty() {
                    self.held_value = platform.pop()
                } else if platform.is_empty() || platform[0] > self.held_value.unwrap() {
                    platform.insert(0, self.held_value.unwrap());
                    self.held_value = None;
                }
            },
            '[' => {
                if self.held_value.is_none() {
                    next_program_counter = self.solution.jump_forward(self.program_counter);
                }
            }
            ']' => {
                if self.held_value.is_some() {
                    next_program_counter = self.solution.jump_back(self.program_counter);
                }
            }
            _ => unreachable!()
        }

        self.tower_position %= self.arrangement.len(); // TODO make this less confusing
        self.program_counter = next_program_counter;

        false
    }
}