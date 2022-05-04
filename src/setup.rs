use dialoguer::{theme::ColorfulTheme, Input};

use crate::config::{self, Config};

pub fn run() {
    let aws_user_id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your aws complete user id?")
        .interact_text()
        .unwrap();

    let aws_profile: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your aws profile to use?")
        .interact_text()
        .unwrap();

    let config = Config {
        aws_user_id,
        aws_profile,
    };

    config::create(config)
}
