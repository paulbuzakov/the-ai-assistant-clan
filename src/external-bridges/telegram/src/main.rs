mod consts;

use crate::consts::TELEGRAM_TOKEN_ENV;
use shared::settings::Settings;

fn main() {
    let settings = Settings::from_env().expect("failed to init settings");

    let token = settings
        .get_value(TELEGRAM_TOKEN_ENV)
        .expect("TELEGRAM_TOKEN must be set");

    println!("TELEGRAM_TOKEN={token}");
}
