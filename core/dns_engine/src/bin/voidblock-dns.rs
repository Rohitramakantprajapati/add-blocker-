use std::path::PathBuf;

use dns_engine::{EngineConfig, resolver};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber with sensible defaults.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut config = EngineConfig::default();

    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "--db" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    config.blocklist_db = PathBuf::from(v);
                } else {
                    eprintln!("--db requires a path");
                    return Ok(());
                }
            }
            "--addr" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    match v.parse() {
                        Ok(addr) => config.bind_addr = addr,
                        Err(_) => {
                            eprintln!("invalid --addr value: {}", v);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("--addr requires an address:port");
                    return Ok(());
                }
            }
            "--cache" => {
                i += 1;
                if let Some(v) = args.get(i) {
                    match v.parse::<usize>() {
                        Ok(n) => config.cache_capacity = n,
                        Err(_) => {
                            eprintln!("invalid --cache value: {}", v);
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("--cache requires a numeric capacity");
                    return Ok(());
                }
            }
            other => {
                eprintln!("unknown option: {}", other);
                print_help();
                return Ok(());
            }
        }
        i += 1;
    }

    tracing::info!(address = %config.bind_addr, db = %config.blocklist_db.display(), "starting VoidBlock DNS resolver");

    // Spawn the resolver; we will gracefully abort on Ctrl+C.
    let cfg = config.clone();
    let handle = tokio::spawn(async move {
        if let Err(e) = resolver::run(cfg).await {
            tracing::error!(error = %format!("{e}"), "resolver exited with error");
        }
    });

    // Wait for shutdown signal
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("shutdown requested via Ctrl+C");
        }
    }

    // Abort the resolver task and wait for it to finish.
    tracing::info!("shutting down resolver task");
    handle.abort();
    let _ = handle.await;

    tracing::info!("shutdown complete");
    Ok(())
}

fn print_help() {
    println!(
        "VoidBlock DNS resolver\n\nUSAGE:\n  voidblock-dns [--db <path>] [--addr <ip:port>] [--cache <entries>] [--help]\n\nOptions:\n  --db <path>     Path to blocklist SQLite DB (default: blocklists/voidblock.db)\n  --addr <addr>   Bind address (default: 127.0.0.1:53)\n  --cache <n>     Response cache capacity (default: 4096)\n  -h, --help      Print this help\n"
    );
}
