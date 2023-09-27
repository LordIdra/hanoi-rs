use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Solution program path
    #[arg(short, long)]
    pub solution: String,

    /// Problem file path
    #[arg(short, long)]
    pub problem: String,
}