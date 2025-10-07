mod client;
mod config;
mod docker;
mod docker_logs;
mod hardware;
mod log_forwarder;
mod logging;
mod metrics;

use clap::Parser;
use tracing::info;

use client::ReplicantClient;
use config::Config;

#[derive(Parser)]
#[command(name = "nexus-replicant")]
#[command(about = "Nexus Replicant - Simple worker node client")]
struct Args {
    /// Path to configuration file
    #[arg(env = "NEXUS_CONFIG", default_value = "conf/replicant.yml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let default_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    let logging_setup = logging::setup(default_level)?;

    info!("Nexus Replicant starting...");

    let args = Args::parse();
    let config = Config::load(&args.config)?;

    info!(
        "Loaded config for replicant: {}",
        config.replicant.replicant_id
    );

    info!("Validating hardware configuration...");
    hardware::print_hardware_config(&config.replicant.hardware);
    hardware::validate_hardware(&config.replicant.hardware)?;
    info!("Hardware validation passed");

    let mut client = ReplicantClient::new(config, logging_setup.container_log_tx).await?;
    client
        .run(logging_setup.unified_log_rx, logging_setup.log_handle)
        .await?;

    Ok(())
}
