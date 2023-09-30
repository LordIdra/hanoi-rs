use std::{sync::{Mutex, Arc, atomic::{AtomicU64, Ordering}}, thread::{self, JoinHandle}};

use crate::{solution::Solution, solution_runner::SolutionRunner};

pub const CHARACTERS: [char; 6] = ['>', '<', '.', '~', '[', ']'];

pub fn recurse(solutions: &Arc<Mutex<Vec<Vec<char>>>>, current_solution: &mut Solution, program: &mut Vec<char>, characters: usize, max_characters: usize, thread_depth: i32, attempts: &AtomicU64) {
    let closure = move |character: &char| {
        program.push(*character);
        run(solutions, current_solution, program, characters+1, max_characters, thread_depth-1, attempts);
        program.pop();
    };
    CHARACTERS.iter().for_each(closure);
}

pub fn recurse_threaded(solutions: &Arc<Mutex<Vec<Vec<char>>>>, program: &[char], characters: usize, max_characters: usize, thread_depth: i32, attempts: &AtomicU64) {
    let mut handles: Vec<JoinHandle<_>> = vec![];
    for character in CHARACTERS {
        let solutions = solutions.clone();
        let mut current_solution = Solution::new(max_characters);
        let mut program = program.to_owned();
        program.push(character);
        let closure = move || {
            run(&solutions, &mut current_solution, &mut program, characters+1, max_characters, thread_depth-1, attempts);
        };
        handles.push(thread::spawn(closure));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn run(solutions: &Arc<Mutex<Vec<Vec<char>>>>, current_solution: &mut Solution, program: &mut Vec<char>, characters: usize, max_characters: usize, thread_depth: i32, attempts: &AtomicU64) {
    if max_characters - characters == 0 {
        attempts.fetch_add(1, Ordering::Relaxed);
        if current_solution.load(program) && SolutionRunner::run(current_solution) {
            solutions.lock().unwrap().push(current_solution.get_program());
        }
        return;
    }

    if thread_depth > 0 {
        recurse_threaded(solutions, program, characters, max_characters, thread_depth, attempts);
    } else {
        recurse(solutions, current_solution, program, characters, max_characters, thread_depth, attempts);
    }
}