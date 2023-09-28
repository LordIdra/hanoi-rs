#[derive(Debug)]
pub struct Solution {
    program: Vec<char>,
    back_jumps: Vec<usize>,
    forward_jumps: Vec<usize>,
}

impl Solution {
    pub fn load(program: Vec<char>) -> Result<Self, ()> {
        // Short circuit if there are not the same number of [ and ]
        let mut branch_counter = 0;
        for character in &program {
            if *character == '[' {
                branch_counter += 1;
            } else if *character == ']' {
                branch_counter -= 1;
            }
        }

        if branch_counter != 0{
            return Err(());
        }

        let mut loop_stack: Vec<usize> = vec![];
        let mut back_jumps: Vec<usize> = vec![0; program.len()];
        let mut forward_jumps: Vec<usize> = vec![0; program.len()];

        // Build jump tables using loop stack
        for i in program.iter().enumerate() {
            match i.1 {
                '<' | '>' | '.' | '~' => (),
                '[' => loop_stack.push(i.0),
                ']' => {
                    let start_index = loop_stack.pop()
                        .ok_or({})?;
                        //.ok_or_else(|| eprintln!("Mismatched ] at index {}", i))?;
                    back_jumps[i.0] = start_index + 1;
                    forward_jumps[start_index] = i.0 + 1;
                }
                _ => {
                    //eprintln!("Invalid token '{}'", character);
                    return Err(());
                }
            }
        }

        if !loop_stack.is_empty() {
            //eprintln!("Unclosed [");
            return Err(())
        }

        Ok(Self { program, back_jumps, forward_jumps })
    }

    pub fn get_token(&self, i: usize) -> Option<char> {
        self.program.get(i).cloned()
    }

    pub fn get_program(&self) -> Vec<char> {
        self.program.clone()
    }

    pub fn jump_forward(&self, program_counter: usize) -> usize {
        self.forward_jumps[program_counter]
    }

    pub fn jump_back(&self, program_counter: usize) -> usize {
        self.back_jumps[program_counter]
    }
}