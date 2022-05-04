use chrono::prelude::{DateTime, Local};
use clap::Parser;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use config::Config;
use copypasta::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::Write;
extern crate dirs;

mod config;
mod setup;

#[derive(Parser, Debug)]
struct Cli {
    command: String,
}

struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let config = config::read();
        check_credentials(&config);
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn main() {
    if !config::exists() {
        setup::run();
    }

    run();
}

fn run() {
    let args = Cli::parse();
    match args {
        _listen => {
            let _ = {
                println!("Started to listen for new credentials");
                Master::new(Handler).run()
            };
        }
    };
}

fn check_credentials(config: &Config) {
    let aws_user_id = format!("[{}]", &config.aws_user_id);

    let credentials = get_credentials_from_clipboard();

    let first_line = credentials.lines().next();
    let needle = &aws_user_id[..];

    if first_line == Some(needle) {
        write_new_credentials(config, &credentials);
    }
}

fn write_new_credentials(config: &Config, credentials: &str) {
    let home_dir = dirs::home_dir();
    let mut credentials_file_path = home_dir.unwrap();
    credentials_file_path.push(".aws");
    credentials_file_path.push("credentials");

    let mut f = File::create(credentials_file_path).expect("Unable to create file");

    let re = Regex::new(r"\[(.*?)\]").unwrap();

    let final_data = re.replace(credentials, format!("[{}]", &config.aws_profile));

    f.write_all(final_data.as_bytes())
        .expect("Unable to write data");

    let local: DateTime<Local> = Local::now();
    println!("{} Wrote the new credentials into credentials file", &local)
}

fn get_credentials_from_clipboard() -> String {
    let mut ctx = ClipboardContext::new().unwrap();

    let data = match ctx.get_contents() {
        Ok(data) => data,
        Err(_) => "".to_string(),
    };

    data
}
