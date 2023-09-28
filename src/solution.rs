use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Solution {
    program: String,
    back_jumps: BTreeMap<usize, usize>,
    forward_jumps: BTreeMap<usize, usize>,
}

impl Solution {
    pub fn load(program: String) -> Result<Self, ()> {
        let mut loop_stack: Vec<usize> = vec![];
        let mut back_jumps: BTreeMap<usize, usize> = BTreeMap::new();
        let mut forward_jumps: BTreeMap<usize, usize> = BTreeMap::new();

        // Build jump tables using loop stack
        for i in 0..program.len() {
            match program.chars().nth(i).unwrap() {
                '<' | '>' | '.' | '~' => (),
                '[' => loop_stack.push(i),
                ']' => {
                    let start_index = loop_stack.pop()
                        .ok_or({})?;
                        //.ok_or_else(|| eprintln!("Mismatched ] at index {}", i))?;
                    back_jumps.insert(i, start_index + 1);
                    forward_jumps.insert(start_index, i + 1);
                }
                character => {
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
        self.program.chars().nth(i)
    }

    pub fn get_program(&self) -> String {
        self.program.clone()
    }

    pub fn jump_forward(&self, program_counter: usize) -> usize {
        self.forward_jumps[&program_counter]
    }

    pub fn jump_back(&self, program_counter: usize) -> usize {
        self.back_jumps[&program_counter]
    }
}