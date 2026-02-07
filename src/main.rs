use clap::Parser;
use console::style;
use dialoguer::Input;
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
                None => Input::<String>::new()
                    .with_prompt(style("What do you want to learn?").cyan().to_string())
                    .interact_text()?,
            };

            let motivation = match args.motivation {
                Some(m) => m,
                None => Input::<String>::new()
                    .with_prompt(
                        style(format!("Why do you want to learn \"{}\"?", topic))
                            .cyan()
                            .to_string(),
                    )
                    .interact_text()?,
            };

            ui.print_checking();

            match manager.create_task(&topic, &motivation).await {
                Err(e) => {
                    if e.to_string() != "Motivation rejected" {
                        eprintln!("\n{} {}", style("Error:").red().bold(), e);
                    }
                }
                Ok(id) => {
                    let smart_goal = Input::<String>::new()
                        .with_prompt("What is your SMART goal?")
                        .interact_text()?;

                    match manager.evaluate_smart_goal(id, &smart_goal).await {
                        Err(e) => {
                            if e.to_string() != "Smart goal rejected" {
                                eprintln!("\n{} {}", style("Error:").red().bold(), e);
                            }
                        }
                        Ok(refined_goal) => {
                            let confirm = Input::<String>::new()
                                .with_prompt(
                                    format!(
                                        "Are you sure you want to update the smart goal? (yes/no)\n\n{}",
                                        style(refined_goal.clone()).bold(),
                                    ),
                                )
                                .interact_text()?;
                            if confirm.to_lowercase().eq("yes") {
                                manager.update_task_smart_goal(id, &refined_goal).await?;
                                println!("Smart goal updated successfully");
                            } else {
                                println!("Smart goal not updated");
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
