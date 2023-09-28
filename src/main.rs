use std::{fs::{self, File}, io::{Write, LineWriter}, time::Instant};

use clap::Parser;
use cli::Args;
use problem::Problem;
use solution::Solution;
use solution_runner::SolutionRunner;

mod cli;
mod problem;
mod solution_runner;
mod solution;

const CHARACTERS: [char; 6] = ['>', '<', '.', '~', '[', ']'];

fn run(problem: Problem, string: String, i: i32, attempts: &mut i32, max_attempts: i32, programs: &mut Vec<String>) {
    if i == 0 {
        if let Ok(solution) = Solution::load(string) {
            if SolutionRunner::run(&solution, problem) {
                programs.push(solution.get_program());
            }
        }

        *attempts += 1;
        if *attempts % 20000 == 0 {
            let percentage =  100.0 * (*attempts as f32) / (max_attempts as f32);
            println!("{}/{} ({}%)", attempts, max_attempts, percentage)
        }

        return;
    }

    for character in CHARACTERS {
        run(problem.clone(), string.clone() + character.to_string().as_str(), i-1, attempts, max_attempts, programs);
    }
}

fn main() -> Result<(), ()> {
    let args = Args::parse();
    let problem = Problem::load(&args)?;
    
    for character_count in 2..10 {
        let start_time = Instant::now();
        let max_attempts = i32::pow(CHARACTERS.len() as i32, character_count as u32 - 1);
        let mut attempts = 0;
        let mut programs = vec![];
        println!("=============");
        println!("{} characters", character_count);
        println!("=============");
        run(problem.clone(), ".".to_string(), character_count-1, &mut attempts, max_attempts, &mut programs);
        let time_taken = Instant::now() - start_time;
        println!("{} characters complete in {}s with {} solutions", character_count, time_taken.as_secs_f32(), programs.len());
        println!("\n");

        let file = File::create(format!("all_solutions/{}", character_count)).unwrap();
        let mut writer = LineWriter::new(file);
        for program in programs {
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