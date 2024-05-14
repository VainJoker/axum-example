pub mod constants;

use std::sync::Arc;

use tokio::signal;

use crate::library::{dber::DB, error::AppResult, Dber, Mqer, Redis, Redisor};

pub struct AppState {
    pub db: Dber,
    pub redis: Redisor,
    pub mq: Mqer,
}

impl AppState {
    pub async fn init() -> Self {
        Self {
            db: Dber::init().await,
            redis: Redisor::init(),
            mq: Mqer::init(),
        }
    }

    pub const fn get_db(&self) -> &DB {
        &self.db.pool
    }

    pub async fn get_redis(&self) -> AppResult<Redis> {
        Ok(self.redis.get_conn().await?)
    }

    pub const fn get_mq(&self) -> AppResult<&Mqer> {
        Ok(&self.mq)
    }
}

pub async fn shutdown_signal(state: Arc<AppState>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            tracing::info!("Ctrl+C signal received.");
            match state.mq.graceful_shutdown(){
                Ok(_) => (),
                Err(e) => tracing::error!("Error shutting down RabbitMQ: {:?}", e),
            }
        },
        () = terminate => {
            tracing::info!("Terminate signal received.");
            match state.mq.graceful_shutdown(){
                Ok(_) => (),
                Err(e) => tracing::error!("Error shutting down RabbitMQ: {:?}", e),
            }
        },
    }
}
