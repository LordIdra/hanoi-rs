use std::{fs::File, io::{Write, LineWriter}, time::Instant, sync::{Mutex, Arc}};

use async_recursion::async_recursion;
use clap::Parser;
use cli::Args;
use futures::{future::join_all, executor::block_on};
use problem::Problem;
use solution::Solution;
use solution_runner::SolutionRunner;

mod cli;
mod problem;
mod solution_runner;
mod solution;

const CHARACTERS: [char; 6] = ['>', '<', '.', '~', '[', ']'];

#[async_recursion]
async fn run(problem: Problem, string: String, i: i32, attempts: Arc<Mutex<i32>>, max_attempts: i32, programs: Arc<Mutex<Vec<String>>>) {
    if i == 0 {
        if let Ok(solution) = Solution::load(string) {
            if SolutionRunner::run(&solution, problem) {
                programs.lock().unwrap().push(solution.get_program());
            }
        }

        let mut attempts_guard = *attempts.lock().unwrap();
        attempts_guard += 1;
        if attempts_guard % 100000 == 0 {
            let percentage = 100.0 * (attempts_guard as f32) / (max_attempts as f32);
            println!("{}/{} ({}%)", attempts_guard, max_attempts, percentage)
        }

        return;
    }

    let mut threads = vec![];

    for character in CHARACTERS {
        threads.push(run(problem.clone(), string.clone() + character.to_string().as_str(), i-1, attempts.clone(), max_attempts, programs.clone()));
    }

    join_all(threads).await;
}

fn main() -> Result<(), ()> {
    let args = Args::parse();
    let problem = Problem::load(&args)?;
    
    for character_count in 2..12 {
        let start_time = Instant::now();
        let max_attempts = i32::pow(CHARACTERS.len() as i32, character_count as u32 - 1);
        let attempts = Arc::new(Mutex::new(0));
        let programs = Arc::new(Mutex::new(vec![]));
        println!("=============");
        println!("{} characters", character_count);
        println!("=============");

        block_on(run(problem.clone(), ".".to_string(), character_count-1, attempts, max_attempts, programs.clone()));

        let time_taken = Instant::now() - start_time;
        println!("{} characters complete in {}s with {} solutions", character_count, time_taken.as_secs_f32(), programs.lock().unwrap().len());
        println!("\n");

        let file = File::create(format!("all_solutions/{}", character_count)).unwrap();
        let mut writer = LineWriter::new(file);
        for program in programs.lock().unwrap().iter() {
            writer.write_all(program.as_bytes()).expect("Failed to write to file");
            writer.write_all(b"\n").expect("Failed to write to file");
        }
    }
    
    // Load program and strip whitespace
    // let program = fs::read_to_string(args.solution.clone())
    //     .map_err(|err| eprintln!("Could not read file at '{}' with error '{}'", args.solution, err))?
    //     .replace([' ', '\n'], "");

    // let solution = Solution::load(".[>.>.>.>.".to_string())?;
    // SolutionRunner::run(solution, problem);
    Ok(())
}