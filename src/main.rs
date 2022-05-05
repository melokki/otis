use clap::Parser;

extern crate dirs;

mod commands;
mod config;
mod setup;

#[derive(Parser, Debug)]
enum Commands {
    Listen,
    Deploy,
}

fn main() {
    if !config::exists() {
        setup::run();
    }

    run();
}

fn run() {
    let args = Commands::parse();

    match args {
        Commands::Listen => {
            println!("Started to listen for new credentials");
            commands::listen();
        }
        Commands::Deploy => {
            println!("Start the deployment process");
            commands::deploy();
        }
    };
}
