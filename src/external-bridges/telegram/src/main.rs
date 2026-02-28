mod consts;

use crate::consts::TELEGRAM_TOKEN_ENV;
use shared::logger;
use shared::settings::Settings;

fn main() {
    logger::init();

    let settings = Settings::from_env().expect("failed to init settings");

    let token = match settings.get_value(TELEGRAM_TOKEN_ENV) {
        Ok(t) => t,
        Err(e) => {
            logger::error!("{e}");
            return;
        }
    };

    for i in 1..=1000 {
        logger::info!("TELEGRAM_TOKEN={} {}", token, i);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}