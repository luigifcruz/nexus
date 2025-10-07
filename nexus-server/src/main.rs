mod app;
mod application;

mod domain;
mod repositories;
mod transport;

use std::error::Error;
use std::io::BufReader;
use std::sync::{mpsc, Arc, Mutex};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport as tonic_transport;
use tonic_web::GrpcWebLayer;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;

use app::Container;
use clap::Parser;
use serde_yaml;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        env = "NEXUS_CONFIG",
        value_name = "config",
        help = "Path for the NEXUS configuration file."
    )]
    pub config: String,

    #[arg(long, default_value = "0.0.0.0", help = "Host address to listen on.")]
    pub host: String,

    #[arg(long, default_value = "50051", help = "Port number to listen on.")]
    pub port: u16,
}

#[derive(Debug, Default)]
struct Config {
    version: String,
    scheduler_interval: u64,
}

impl Config {
    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        info!("Opening configuration file: '{}'", path);

        if !std::path::Path::new(path).exists() {
            return Err("Configuration file does not exist.".into());
        }

        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let yaml: serde_yaml::Value = serde_yaml::from_reader(reader)?;

        // Extract configuration values
        let version = yaml
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0")
            .to_string();

        if version != "1.0" {
            return Err("Configuration file version mismatch.".into());
        }

        let scheduler_interval = yaml
            .get("routines")
            .and_then(|r| r.get("instance"))
            .and_then(|i| i.get("interval"))
            .and_then(|v| v.as_u64())
            .unwrap_or(5);

        Ok(Config {
            version: env!("CARGO_PKG_VERSION").to_string(),
            scheduler_interval,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let default_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level)),
        )
        .init();

    info!("Welcome to Nexus!");

    // Parse CLI arguments
    let args = Args::parse();

    // Load configuration
    let config = Config::from_file(&args.config)?;

    // Create dependency injection container
    let container = Arc::new(Container::new(
        config.version.clone(),
        config.scheduler_interval,
    )?);

    // Initialize container
    container.initialize().await?;

    // Create TCP listener
    let addr = format!("{}:{}", args.host, args.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Server listening on {}.", addr);

    // Create shutdown channel
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    // Run schedulers in background
    let scheduler_container = Arc::clone(&container);
    let scheduler_shutdown_rx = Arc::new(Mutex::new(shutdown_rx));
    let scheduler_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(
            scheduler_container.scheduler_interval(),
        ));

        loop {
            // Check for shutdown signal
            if let Ok(rx) = scheduler_shutdown_rx.lock() {
                if rx.try_recv().is_ok() {
                    info!("Scheduler received shutdown signal");
                    break;
                }
            }

            interval.tick().await;

            // Run scheduled tasks
            if let Err(e) = scheduler_container.run_schedulers().await {
                tracing::error!("Error running schedulers: {}", e);
            }
        }

        // Shutdown scheduler
        let _ = scheduler_container.shutdown().await;
        Ok::<(), Box<dyn Error + Send + Sync>>(())
    });

    // Install signal handler
    let shutdown_tx_clone = shutdown_tx.clone();
    ctrlc::set_handler(move || {
        info!("\nReceived stop signal.");
        let _ = shutdown_tx_clone.send(());
    })?;

    // Create gRPC handlers
    let meta_handler = container.create_meta_handler();
    let replicant_handler = container.create_replicant_handler();
    let nexus_handler = container.create_nexus_handler();

    // Create reflection service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(transport::ReplicantFileDescriptorSet)
        .register_encoded_file_descriptor_set(transport::MetaFileDescriptorSet)
        .register_encoded_file_descriptor_set(transport::NexusFileDescriptorSet)
        .build()?;

    // Create and run gRPC server
    let server_handle = tokio::spawn(async move {
        let result = tonic_transport::Server::builder()
            .accept_http1(true)
            .layer(tower_http::cors::CorsLayer::permissive())
            .layer(GrpcWebLayer::new())
            .add_service(reflection_service)
            .add_service(transport::ReplicantServer::new(replicant_handler))
            .add_service(transport::MetaServer::new(meta_handler))
            .add_service(transport::NexusServer::new(nexus_handler))
            .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
                // Wait for shutdown signal
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to listen for ctrl+c");
                info!("Server shutting down...");
            })
            .await;

        match result {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    });

    // Wait for either task to complete
    tokio::select! {
        scheduler_result = scheduler_handle => {
            match scheduler_result {
                Ok(Ok(())) => info!("Schedulers shut down successfully"),
                Ok(Err(e)) => {
                    eprintln!("Scheduler error: {}", e);
                    let _ = shutdown_tx.send(());
                    return Err(format!("Scheduler error: {}", e).into());
                }
                Err(e) => {
                    eprintln!("Scheduler task panicked: {}", e);
                    return Err(format!("Scheduler task panicked: {}", e).into());
                }
            }
        }
        server_result = server_handle => {
            match server_result {
                Ok(Ok(())) => info!("Server shut down successfully"),
                Ok(Err(e)) => {
                    eprintln!("Server error: {}", e);
                    let _ = shutdown_tx.send(());
                    return Err(format!("Server error: {}", e).into());
                }
                Err(e) => {
                    eprintln!("Server task panicked: {}", e);
                    return Err(format!("Server task panicked: {}", e).into());
                }
            }
        }
    }

    // Final cleanup
    if let Err(e) = container.shutdown().await {
        eprintln!("Error during shutdown: {}", e);
    }

    info!("Goodbye!");

    Ok(())
}
