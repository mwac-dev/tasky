use clap::{Parser, Subcommand};

use crate::model::{Difficulty, Priority};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        #[arg(long)]
        priority: Option<Priority>,
        #[arg(long)]
        difficulty: Option<Difficulty>,
    },
    Remove {
        title: String,
    },
    Complete {
        title: String,
    },
    List,
    Suggest,
}
