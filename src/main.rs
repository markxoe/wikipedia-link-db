use clap::{Parser, Subcommand};

mod commands;
mod data;
mod indication;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<commands::Commands>,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    DeriveDB {
        /// Path to the page.sql file
        #[arg(short, long)]
        pages_sql: String,

        /// Path to the redirects.sql file
        #[arg(short, long)]
        redirects_sql: String,

        /// Path to the links.sql file
        #[arg(short, long)]
        links_sql: String,

        /// Output Path
        #[arg(short, long)]
        output: String,
    },

    Test {
        #[arg(short, long)]
        db: String,
    },
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Some(command) => command.execute(),
        None => {
            println!("No command specified");
        }
    }
}
