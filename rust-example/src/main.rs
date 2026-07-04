//! Binary entrypoint.

use booking_a_stay::error::StartupError;
use booking_a_stay::{app, config, telemetry};

#[tokio::main]
async fn main() -> Result<(), StartupError> {
    let config = config::Config::load()?;
    telemetry::init(&config.telemetry);

    app::run(config).await
}
