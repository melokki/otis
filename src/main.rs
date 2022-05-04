use chrono::prelude::{DateTime, Local};
use clap::Parser;
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use copypasta::{ClipboardContext, ClipboardProvider};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Parser, Debug)]
struct Cli {
    command: String,
}

struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        check_credentials();
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn main() {
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

fn check_credentials() {
    //TODO get the neaddle from a config file
    let needle = Some("");

    let credentials = get_credentials_from_clipboard();

    let first_line = credentials.lines().next();

    if first_line == needle {
        write_new_credentials(&credentials);
    }
}

fn write_new_credentials(credentials: &str) {
    let mut f = File::create("~/.aws/credentials").expect("Unable to create file");

    let re = Regex::new(r"\[(.*?)\]").unwrap();

    // TODO get the profile from a config file
    let final_data = re.replace(credentials, "");

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
