mod env;

use clap::{Args, Parser, Subcommand, ValueEnum};
use env::Environment;

#[derive(ValueEnum, Clone, Debug, Copy)]
enum Driver {
    Mamba,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Adds files to mysdapp
    Create(CreateArguments),
}

#[derive(Args, Debug, Clone)]
struct CreateArguments {
    driver: Driver,
}

fn get_driver(driver: Driver) -> impl env::Environment {
    return match driver {
        Driver::Mamba => env::Mamba {},
    };
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Create(args) => {
            get_driver(args.driver).create();
        }
    }
}
