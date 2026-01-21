use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// This is the data structure we will share between Client and Server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemMetric {
    pub host_id: String,
    pub cpu_usage: f32,
    pub ram_usage_mb: f32,       
    pub disk_usage_percent: f32, 
    pub net_rx_kb: f32,          
    pub net_tx_kb: f32,   
    pub gpu_usage: f32,       
    pub gpu_temp: f32,        
    pub gpu_vram_used_mb: f32,      
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub link: Option<String>,
}

#[derive(Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub question: String,
}

#[derive(Deserialize)]
pub struct SeedRequest {
    pub facts: Vec<String>, // e.g., ["Nick is a Sim Engineer", "Nick knows Rust"]
}
