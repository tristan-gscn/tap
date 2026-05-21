mod config;
mod logger;
mod network;
mod protocol;
mod state;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

const LISTEN_ADDR: &str = "127.0.0.1:4000";

/// Entry point for the TAP Server.
/// Initializes the logger, loads the world configuration, and starts the network listener.
#[tokio::main]
async fn main() {
    logger::init();
    info!("Server starting...");

    let cfg = match config::init_global("world.yml") {
        Ok(cfg) => cfg,
        Err(err) => {
            error!(error = %err, "Failed to initialize config");
            std::process::exit(1);
        }
    };

    let problems = config::validate(cfg);
    if !problems.is_empty() {
        for problem in &problems {
            error!(problem = %problem, "World validation error");
        }
        error!(
            count = problems.len(),
            "World data is invalid; refusing to start"
        );
        std::process::exit(1);
    }
    info!(
        rooms = cfg.world.locations.len(),
        npcs = cfg.world.npcs.len(),
        items = cfg.world.items.len(),
        quests = cfg.world.quests.len(),
        "World loaded and validated"
    );

    let game_state = Arc::new(RwLock::new(state::game::GameState::new()));

    network::listener::start(LISTEN_ADDR, game_state).await;
}
