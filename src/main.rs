use std::{fs::File, io::{Write, LineWriter}, time::Instant, sync::{Mutex, Arc}};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use solution::Solution;
use solution_runner::SolutionRunner;

mod problem;
mod solution_runner;
mod solution;

const CHARACTERS: [char; 6] = ['>', '<', '.', '~', '[', ']'];

fn run(program: Vec<char>, i: i32, programs: Arc<Mutex<Vec<Vec<char>>>>) {
    if i == 0 {
        if let Ok(solution) = Solution::load(program) {
            if SolutionRunner::run(&solution) {
                programs.lock().unwrap().push(solution.get_program());
            }
        }
        return;
    }

    let closure = |character: &char| {
        let mut program = program.clone();
        program.push(*character);
        run(program, i-1, programs.clone());
    };

    CHARACTERS.par_iter().for_each(closure);
}

fn main() -> Result<(), ()> {
    for character_count in 13..16 {
        let start_time = Instant::now();
        let max_attempts = i64::pow(CHARACTERS.len() as i64, character_count as u32 - 1);
        let programs = Arc::new(Mutex::new(vec![]));
        println!("Starting {} characters", character_count);

        run(vec!['.'], character_count - 1, programs.clone());

        let time_taken = Instant::now() - start_time;
        let attempts_per_second = (max_attempts as f64 / time_taken.as_millis() as f64) as i64 * 1000;
        println!("{} characters complete in {}s with {} solutions ({} attempts/s)", character_count, time_taken.as_secs_f32(), programs.lock().unwrap().len(), attempts_per_second);
        println!("\n");

        let file = File::create(format!("all_solutions/{}", character_count)).unwrap();
        let mut writer = LineWriter::new(file);
        for program in programs.lock().unwrap().iter() {
            let program = String::from_iter(program);
            writer.write_all(program.as_bytes()).expect("Failed to write to file");
            writer.write_all(b"\n").expect("Failed to write to file");
        }
    }

    Ok(())
}