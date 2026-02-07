pub mod ui;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author = "hedon",
    about = "Cognitive Gatekeeper",
    long_about = "Gewu is a cognitive gatekeeper that helps you clarify your learning motivation and turn it into a specific SMART goal.",
    version = "0.1.0"
)]
pub struct Gewu {
    #[command(subcommand)]
    pub operation: Operation,
}

#[derive(Subcommand)]
pub enum Operation {
    /// Add a new learning task
    Add(AddArgs),
}

#[derive(Args)]
pub struct AddArgs {
    /// The topic of the learning task
    #[arg(short, long)]
    pub topic: Option<String>,

    /// The motivation of the learning task
    #[arg(short, long)]
    pub motivation: Option<String>,
}
