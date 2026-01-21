
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, env, sync::Arc};
use tower_http::services::ServeFile;
use dotenvy::dotenv;
use tokio::sync::broadcast;
use tera::Tera;

// DECLARE MODULES
mod state;
mod handlers;
mod ai;
mod seeder;

use state::AppState;
use crate::ai::Brain;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations automatically on startup
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let (tx, _rx) = broadcast::channel(100);
    let templates = Tera::new("templates/**/*").expect("Failed to parse templates");
    let templates = Arc::new(templates);
    let brain = Arc::new(Brain::new());

    let site_state = AppState { 
        pool, 
        tx, 
        templates,
        brain
    };

    seeder::auto_seed(&site_state).await;

    let app = Router::new()
        // Serve static files    
        //.nest_service("/static", ServeFile::new("static")) 
        .route("/static/style.css", get(handlers::serve_css)) 
        .route_service("/static/monitor.js", ServeFile::new("static/monitor.js"))
        .route_service("/static/portfolio.js", ServeFile::new("static/portfolio.js"))

        // Page Routes
        .route("/", get(handlers::render_home))
        .route("/monitor", get(handlers::render_monitor))
        .route("/about", get(handlers::render_about))
        .route("/resume", get(handlers::render_resume))        
        .route("/chat", post(handlers::chat_handler))

        // API Routes
        .route("/ingest", post(handlers::ingest_metric))
        .route("/history", get(handlers::get_metrics))
        .route("/events", get(handlers::stream_metrics))
        .route("/contact", get(handlers::render_contact))  
        .route("/contact", post(handlers::submit_contact))
        .route("/seed", post(handlers::seed_knowledge))

        .with_state(site_state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}