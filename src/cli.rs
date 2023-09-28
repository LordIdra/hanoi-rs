use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Problem file path
    #[arg(short, long)]
    pub problem: String,
}