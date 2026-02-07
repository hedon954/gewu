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

            if let Err(e) = manager.create_task(&topic, &motivation).await
                && e.to_string() != "Motivation rejected"
            {
                eprintln!("\n{} {}", style("Error:").red().bold(), e);
            }
        }
    }
    Ok(())
}
