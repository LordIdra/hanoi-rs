#[derive(Clone, Debug)]
pub struct Solution {
    program: Vec<char>,
    back_jumps: Vec<usize>,
    forward_jumps: Vec<usize>,
}

impl Solution {
    pub fn new(length: usize) -> Self {
        Self { 
            program: vec![' '; length], 
            back_jumps: vec![0; length], 
            forward_jumps: vec![0; length],
        }
    }

    fn is_valid(program: &Vec<char>) -> bool {
        let mut branch_counter = 0;
        let mut ptr = program.as_ptr();
        
        // Critical path - raw pointers offer a (very) small speedup
        let end = unsafe { ptr.add(program.len()) };
        while ptr != end {
            unsafe {
                if *ptr == '[' {
                    branch_counter += 1;
                } else if *ptr == ']' {
                    branch_counter -= 1;
                    if branch_counter < 0 {
                        return false;
                    }
                }
                ptr = ptr.add(1);
            }
        }

        branch_counter == 0
    }

    // Returns true if valid program
    pub fn load(&mut self, program: &Vec<char>) -> bool {
        // Short circuit if there are not the same number of [ and ]
        if !Self::is_valid(program) {
            return false;
        }

        let mut loop_stack: Vec<usize> = vec![];

        // Build jump tables using loop stack
        for i in program.iter().enumerate() {
            match i.1 {
                '<' | '>' | '.' | '~' => (),
                '[' => loop_stack.push(i.0),
                ']' => {
                    let start_index = loop_stack.pop();
                    if start_index.is_none() {
                        return false;
                    }
                    let start_index = start_index.unwrap();
                    self.back_jumps[i.0] = start_index + 1;
                    self.forward_jumps[start_index] = i.0 + 1;
                }
                _ => {
                    //eprintln!("Invalid token '{}'", character);
                    return false;
                }
            }
        }

        if !loop_stack.is_empty() {
            return false
        }

        self.program = program.clone();
        true
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