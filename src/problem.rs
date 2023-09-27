use std::fs;
use serde_json::Value;

use crate::cli::Args;

pub type Arrangement = Vec<Vec<i32>>;

#[derive(Debug)]
pub struct Problem {
    initial_arrangement: Arrangement,
    goals: Vec<Arrangement>,
}

impl Problem {
    fn load_i32(args: &Args, value: &Value) -> Result<i32, ()> {
        let value_number = value
            .as_number()
            .ok_or_else(|| eprintln!("Expected numberin '{}'", args.problem))?
            .as_i64()
            .ok_or_else(|| eprintln!("Expected integerin '{}'", args.problem))?;
        Ok(value_number as i32)
    }

    fn load_platform(args: &Args, value: &Value) -> Result<Vec<i32>, ()> {
        value
            .as_array()
            .ok_or_else(|| eprintln!("Expected arrayin '{}'", args.problem))?
            .iter()
            .map(|x| Self::load_i32(args, x))
            .collect()
    }

    fn load_arrangement(args: &Args, value: &Value) -> Result<Arrangement, ()> {
        value
            .as_array()
            .ok_or_else(|| eprintln!("Expected arrayin '{}'", args.problem))?
            .iter()
            .map(|x| Self::load_platform(args, x))
            .collect()
    }

    fn load_initial_arrangement(args: &Args, config: &Value) -> Result<Arrangement, ()> {
        let init = config.get("init")
            .ok_or_else(|| eprintln!("Could not find key 'init'in '{}'", args.problem))?;
        Self::load_arrangement(args, init)
    }

    fn load_goals(args: &Args, config: &Value) -> Result<Vec<Arrangement>, ()> {
        config["goals"]
            .as_array()
            .ok_or_else(|| eprintln!("Could not find key 'goals'in '{}'", args.problem))?
            .iter()
            .map(|x| Self::load_arrangement(args, x))
            .collect()
    }

    pub fn load(args: &Args) -> Result<Problem, ()> {
        let config_text = fs::read_to_string(args.problem.clone())
            .map_err(|err| eprintln!("Could not read file at '{}' with error '{}'", args.problem, err))?;
        let config = serde_json::from_str(config_text.as_str())
            .map_err(|_| eprintln!("Could not parse json in '{}'", args.problem))?;
        let initial_arrangement = Self::load_initial_arrangement(args, &config)?;
        let goals = Self::load_goals(args, &config)?;

        Ok(Problem { initial_arrangement, goals })
    }

    pub fn get_initial_arrangement(&self) -> Arrangement {
        self.initial_arrangement.clone()
    }

    pub fn is_final_arrangement(&self, arrangement: &Arrangement) -> bool {
        self.goals.contains(arrangement)
    }
}