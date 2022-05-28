use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "lib_lang")]
#[clap(bin_name = "lib_lang")]
#[clap(about = "dwmadwa", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Run {
        #[clap(long, parse(from_os_str))]
        path: PathBuf,
    },

    #[clap(arg_required_else_help = true)]
    Build {
        #[clap(long, parse(from_os_str))]
        path: PathBuf,
    }
}

pub fn get_path() -> (bool, PathBuf) {
    let args = Cli::parse();

    let path = match args.command {
        Commands::Build { path } => {
            if path.extension().unwrap() != "lib" {
                panic!("Wrong file extension!");
            }

            (false, path)
        },
        Commands::Run { path } => {
            if path.extension().unwrap() != "lib" {
                panic!("Wrong file extension!");
            }

            (true, path)
        }
    };

    path
}