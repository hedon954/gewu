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
    /// Describe a learning task
    Describe(DescribeArgs),
    /// List all learning tasks
    List,
    /// Delete a learning task
    Delete(DeleteArgs),
    /// Set a SMART goal for an existing learning task that was previously created without one.
    /// This follows the same process as the `Add` operation,
    /// but is intended for cases where adding a task was interrupted before setting a SMART goal.
    Plan(PlanArgs),
    /// Record learning progress. The LLM will automatically verify if your submission aligns with active tasks.
    Record(RecordArgs),
    /// Ask the llm to guide you on how to learn the given task.
    Guide(GuideArgs),
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

#[derive(Args)]
pub struct DescribeArgs {
    /// The id of the learning task
    pub id: i64,
}

#[derive(Args)]
pub struct DeleteArgs {
    /// The id of the learning task
    pub id: i64,
}

#[derive(Args)]
pub struct PlanArgs {
    /// The id of the learning task
    pub id: i64,
}

#[derive(Args)]
pub struct RecordArgs {
    /// The content of the learning record
    pub content: String,
}

#[derive(Args)]
pub struct GuideArgs {
    /// The id of the learning task
    pub id: i64,
}
