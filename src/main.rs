use clap::Parser;
use cli::Args;
use problem::Problem;
use solution::Solution;
use solution_runner::SolutionRunner;

mod cli;
mod problem;
mod solution_runner;
mod solution;


fn main() -> Result<(), ()> {
    let args = Args::parse();
    let problem = Problem::load(&args)?;
    let solution = Solution::load(&args)?;
    SolutionRunner::run(solution, problem);
    Ok(())
}