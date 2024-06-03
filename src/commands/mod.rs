use clap::Subcommand;

pub mod derive_db;
pub mod interactive;

#[derive(Subcommand, Debug)]
pub enum Commands {
    DeriveDB {
        #[command(flatten)]
        args: derive_db::DeriveDbArgs,
    },

    Interactive {
        #[command(flatten)]
        args: interactive::InteractiveArgs,
    },
}

pub trait ArgExecutor {
    fn execute(&self);
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::DeriveDB { args } => args.execute(),
            Commands::Interactive { args } => args.execute(),
        }
    }
}
