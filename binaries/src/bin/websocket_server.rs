#![allow(unused_crate_dependencies)]
use std::net::Ipv4Addr;

use clap::Parser;
use server::{Result, run_websocket_server, run_websocket_server_with_tls};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Server address (e.g., 0.0.0.0)
    #[arg(long)]
    address: Ipv4Addr,

    /// Server port (e.g., 8000)
    #[arg(long)]
    port: u16,

    /// Compression level for WebSocket connections.
    /// Accepts values in the range `0..=9`.
    /// * `0` – compression disabled.
    /// * `1` – fastest compression, low compression ratio (default).
    /// * `9` – slowest compression, highest compression ratio.
    ///
    /// The level is passed to `flate2::Compression::new(level)`; see the
    /// documentation for <https://docs.rs/flate2/1.1.2/flate2/struct.Compression.html#method.new> for more info.
    #[arg(long)]
    websocket_compression_level: Option<u32>,

    /// Path to TLS certificate file (enables HTTPS/WSS)
    #[arg(long)]
    cert_file: Option<String>,

    /// Path to TLS private key file (enables HTTPS/WSS)
    #[arg(long)]
    key_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let full_address = format!("{}:{}", args.address, args.port);
    let compression_level = args.websocket_compression_level.unwrap_or(/* Some compression */ 1);

    match (args.cert_file, args.key_file) {
        (Some(cert_file), Some(key_file)) => {
            println!("Running secure websocket server on wss://{full_address}");
            run_websocket_server_with_tls(&full_address, true, compression_level, Some(&cert_file), Some(&key_file)).await?;
        }
        (Some(_), None) => {
            eprintln!("Error: --cert-file specified but --key-file is missing");
            std::process::exit(1);
        }
        (None, Some(_)) => {
            eprintln!("Error: --key-file specified but --cert-file is missing");
            std::process::exit(1);
        }
        (None, None) => {
            println!("Running websocket server on ws://{full_address}");
            run_websocket_server(&full_address, true, compression_level).await?;
        }
    }

    Ok(())
}
