mod controller;

use std::path::PathBuf;
use std::env;

use controller::Controller;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut controller = Controller::new();

    // Load the BPF object if present.
    let default_obj = PathBuf::from("/etc/voidblock/xdp_filter.o");
    if default_obj.exists() {
        if let Err(e) = controller.load_object(&default_obj) {
            tracing::error!(error = %e, "failed to load BPF object");
        }
    }

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("insert") => {
            if let Some(hex) = args.get(2) {
                if let Ok(hash) = u64::from_str_radix(hex.trim_start_matches("0x"), 16) {
                    if let Err(e) = controller.insert_hash(hash) {
                        tracing::error!(error = %e, "insert failed");
                    }
                } else {
                    tracing::warn!(hash = %hex, "invalid hash");
                }
            } else {
                tracing::info!("usage: voidblock insert <hex-hash>");
            }
        }
        _ => {
            tracing::info!(loaded_hashes = controller.blocked.len(), "VoidBlock Linux controller ready");
        }
    }
}
