pub mod commands;
pub mod types;
pub mod helpers;

use clap::{Parser, Subcommand};
use crate::commands::switch::switch_env;

#[derive(Parser, Debug)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Переключение между проектами")]
    Switch {
        #[clap(help = "ID проекта для переключения")]
        project_id: String,
    }
}

#[tokio::main]
async fn main() {
    let parsed = CLI::parse();

    match &parsed.command {
        Some(Commands::Switch { project_id }) => {
            switch_env(project_id.as_str()).await;
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
