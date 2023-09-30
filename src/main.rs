use std::{fs::File, io::{Write, LineWriter}, time::Instant, sync::{Mutex, Arc, atomic::{AtomicU64, Ordering}}};

use solution_runner::SolutionRunner;

use crate::solution::Solution;

mod problem;
mod solution_runner;
mod solution;

pub const CHARACTERS: [char; 6] = ['>', '<', '.', '~', '[', ']'];

fn is_program_valid_intermediate(program: &mut Vec<char>, ) {

}

fn run_single_thread(remaining_characters: usize, program: &mut Vec<char>, current_solution: &mut Solution, solutions: &Arc<Mutex<Vec<Vec<char>>>>, attempts: &AtomicU64) {
    if remaining_characters == 0 {
        attempts.fetch_add(1, Ordering::Relaxed);
        if current_solution.load(program) && SolutionRunner::run(current_solution) {
            solutions.lock().unwrap().push(current_solution.get_program());
        }
        return;
    }

    let closure = move |character: &char| {
        program.push(*character);
        run_single_thread(remaining_characters-1, program, current_solution, solutions, attempts);
        program.pop();
    };
    CHARACTERS.iter().for_each(closure);
}

fn run(character_count: usize) -> (Arc<Mutex<Vec<Vec<char>>>>, u64) {
    let solutions = Arc::new(Mutex::new(vec![]));
    let mut attempts = AtomicU64::new(0);
    
    for character_1 in CHARACTERS {
        for character_2 in CHARACTERS {
            let mut program = vec![character_1, character_2];
            let mut current_solution = Solution::new(character_count);
            run_single_thread(character_count-2, &mut program, &mut current_solution, &solutions, &attempts);
        }
    }

    (solutions, *attempts.get_mut())
}

fn main() -> Result<(), ()> {
    for character_count in 11..12 {
        let start_time = Instant::now();
        println!("Starting {} characters", character_count);

        
        let (solutions, attempts) = run(character_count);

        let time_taken = Instant::now() - start_time;
        let attempts_per_second = (attempts as f64 / time_taken.as_millis() as f64) as i64 * 1000;
        println!("character count {}", character_count);
        println!("time taken {}", time_taken.as_secs_f32());
        println!("attempts {}", attempts);
        println!("attempts per second {}", attempts_per_second);
        println!("solutions found {}", solutions.lock().unwrap().len());
        println!("\n");

        let file = File::create(format!("all_solutions/{}", character_count)).unwrap();
        let mut writer = LineWriter::new(file);
        for program in solutions.lock().unwrap().iter() {
            let program = String::from_iter(program);
            writer.write_all(program.as_bytes()).expect("Failed to write to file");
            writer.write_all(b"\n").expect("Failed to write to file");
        }
    }

    Ok(())
}