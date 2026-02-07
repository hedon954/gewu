use std::io::{self, Write};

use clap::Parser;
use console::style;
use dialoguer::Confirm;
use sqlx::PgPool;

use crate::{
    adapters::{deepseek::DeepSeek, postgres_repo::PostgresRepo},
    cli::{Gewu, Operation, ui::UI},
    services::manager::TaskManager,
};

mod adapters;
mod cli;
mod domain;
mod ports;
mod services;

/// Read a line from stdin with a green `> ` prompt.
/// The text stays on screen exactly as the user typed it.
fn read_input() -> anyhow::Result<String> {
    print!("{} ", style(">").green().bold());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    if input.is_empty() {
        anyhow::bail!("Input cannot be empty");
    }
    Ok(input)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;
    let repo = PostgresRepo::new(pool);
    let llm = DeepSeek::try_new(std::env::var("DEEPSEEK_API_KEY")?).await?;
    let mut manager = TaskManager::new(llm, repo);

    let cli = Gewu::parse();
    let ui = UI::new();

    match cli.operation {
        Operation::Add(args) => {
            let topic = match args.topic {
                Some(t) => t,
                None => {
                    println!("\n{}", style("What do you want to learn?").cyan().bold());
                    read_input()?
                }
            };

            let motivation = match args.motivation {
                Some(m) => m,
                None => {
                    println!(
                        "\n{}\n{}",
                        style(format!("Why do you want to learn \"{}\"?", topic))
                            .cyan()
                            .bold(),
                        style("Please describe your specific motivation and use case:").cyan()
                    );
                    read_input()?
                }
            };

            ui.print_checking_motivation();

            match manager.create_task(&topic, &motivation).await {
                Err(e) => {
                    if e.to_string() != "Motivation rejected" {
                        eprintln!("\n{} {}", style("Error:").red().bold(), e);
                    }
                }
                Ok(id) => {
                    // Loop until user provides an approved SMART goal
                    loop {
                        println!(
                            "\n{}\n{}",
                            style("What is your SMART goal?").cyan().bold(),
                            style("(Specific, Measurable, Achievable, Relevant, Time-bound)")
                                .cyan()
                        );
                        let smart_goal = read_input()?;

                        ui.print_checking_smart_goal();

                        match manager.evaluate_smart_goal(id, &smart_goal).await {
                            Err(e) => {
                                eprintln!("\n{} {}", style("Error:").red().bold(), e);
                                break;
                            }
                            Ok(verdict) => {
                                if !verdict.passed {
                                    // Rejected: show guidance and loop for re-entry
                                    ui.print_smart_goal_rejected(
                                        &verdict.reason,
                                        &verdict.guidance.unwrap_or_default(),
                                    );
                                    continue;
                                }

                                // Approved: display the refined SMART goal table
                                let refined = verdict.refined_goal.unwrap();
                                ui.print_smart_goal_approved(&verdict.reason, &refined);

                                // Ask for confirmation
                                let confirmed = Confirm::new()
                                    .with_prompt(
                                        style("Accept this refined SMART goal?").cyan().to_string(),
                                    )
                                    .default(true)
                                    .interact()?;

                                if confirmed {
                                    let goal_json = serde_json::to_string(&refined)?;
                                    manager.update_task_smart_goal(id, &goal_json).await?;
                                    ui.print_smart_goal_saved();
                                } else {
                                    ui.print_smart_goal_not_saved();
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
        Operation::Describe(args) => {
            let task = manager.get_task(args.id).await?;
            ui.print_task_detail(&task);
        }
    }
    Ok(())
}
