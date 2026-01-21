use sqlx::{Pool, Postgres};
use tokio::sync::broadcast;
use monitor_shared::SystemMetric;
use std::sync::Arc;
use tera::Tera;
use crate::ai::Brain;

// Make the struct and fields 'pub' so other files can see them
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub tx: broadcast::Sender<SystemMetric>,
    pub templates: Arc<Tera>,
    pub brain: Arc<Brain>,
}