use std::path::PathBuf;

use anyhow::{bail, Result};
use structopt::StructOpt;

use basalt::{list, smr};

#[derive(StructOpt)]
#[structopt(about = "Lavamoat analyzer and bundler")]
enum BasaltCommands {
    /// Print the module graph for entry point(s)
    Ls {
        /// Print the file name for each module
        #[structopt(short, long)]
        include_file: bool,

        /// Module entry point.
        #[structopt(parse(from_os_str))]
        entries: Vec<PathBuf>,
    },

    /// Generate a static module record for a module
    Smr {
        /// Module path
        #[structopt(parse(from_os_str))]
        module: PathBuf,
    },
}

fn main() -> Result<()> {
    if std::env::var("RUST_LOG").ok().is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    let args = BasaltCommands::from_args();
    match args {
        BasaltCommands::Ls {
            entries,
            include_file,
        } => {
            if entries.is_empty() {
                bail!("List command requires entry points.");
            }
            list(entries, include_file)?;
        }

        BasaltCommands::Smr { module } => {
            smr(module)?;
        }
    }
    Ok(())
}
