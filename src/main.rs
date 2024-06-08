use clap::Parser;

mod commands;
mod data;
mod indication;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<commands::Commands>,
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
