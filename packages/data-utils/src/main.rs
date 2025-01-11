use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Check 2 RawDBs to make sure db1 is valid and db2 is the same as db1.
    Check { db1: PathBuf, db2: PathBuf },
    Dump {
        /// Path to save the dumped files
        path: PathBuf,

        /// Dump CompactDB instead of RawDB
        #[clap(short, long)]
        compact: bool,
    },
    ReadTest {
        /// Path to the CompactDB
        path: PathBuf,
    },
}

pub fn main() -> ExitCode {
    if let Err(e) = main_internal() {
        eprintln!("Error: {:?}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn main_internal() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Check { db1, db2 } => {
            botw_recipe_data_utils::check::check_raw_db(&db1)?;
            botw_recipe_data_utils::check::compare_raw_db(&db1, &db2)?;
        }
        Command::Dump { path, compact } => {
            if compact {
                botw_recipe_data_utils::dump::dump_compact_db(&path)?;
            } else {
                botw_recipe_data_utils::dump::dump_raw_db(&path)?;
            }
        }
        Command::ReadTest { path } => {
            botw_recipe_data_utils::readtest::test_read_db(&path)?;
        }
    }
    Ok(())
}
