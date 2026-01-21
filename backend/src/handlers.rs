use axum::{
    extract::{State},
    response::{Html, sse::{Event, Sse}, IntoResponse},
    Json,
    Form,
};
use axum::http::{header, StatusCode, HeaderMap}; // For manual CSS serving
use futures::stream::Stream;
use monitor_shared::{SystemMetric, Project, ContactForm, ChatRequest, SeedRequest};
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use std::env;
use crate::state::AppState;
use pgvector::Vector;

// ---------------------------------------------------------
// PAGE RENDERERS
// ---------------------------------------------------------
pub async fn render_home(State(state): State<AppState>) -> Html<String> {
    let projects = sqlx::query_as!(
        Project,
        "SELECT id, title, description, link FROM projects ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_else(|_| vec![]);

    let mut context = tera::Context::new();
    context.insert("projects", &projects);
    let rendered = state.templates.render("index.html", &context).unwrap();
    Html(rendered)
}

pub async fn render_monitor(State(state): State<AppState>) -> Html<String> {
    let context = tera::Context::new();
    let rendered = state.templates.render("monitor.html", &context).unwrap();
    Html(rendered)
}

pub async fn render_about(State(state): State<AppState>) -> Html<String> {
    let context = tera::Context::new();
    let rendered = state.templates.render("about.html", &context).unwrap();
    Html(rendered)
}

pub async fn render_resume(State(state): State<AppState>) -> Html<String> {
    let context = tera::Context::new();
    let rendered = state.templates.render("resume.html", &context).unwrap();
    Html(rendered)
}

// ---------------------------------------------------------
// API HANDLERS
// ---------------------------------------------------------
pub async fn ingest_metric(
    State(state): State<AppState>,
    Json(payload): Json<SystemMetric>
) {
    println!("Received: {:.2}% CPU", payload.cpu_usage);
    let _ = sqlx::query!(
        "INSERT INTO metrics (host_id, cpu_usage, ram_usage_mb, disk_usage_percent, net_rx_kb, net_tx_kb, gpu_usage, gpu_temp, gpu_vram_used_mb, timestamp) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        payload.host_id, 
        payload.cpu_usage, 
        payload.ram_usage_mb, 
        payload.disk_usage_percent,
        payload.net_rx_kb,
        payload.net_tx_kb,
        payload.gpu_usage,
        payload.gpu_temp,
        payload.gpu_vram_used_mb,
        payload.timestamp
    )
    .execute(&state.pool)
    .await;

    let _ = state.tx.send(payload); 
}

pub async fn get_metrics(
    State(state): State<AppState>
) -> Json<Vec<SystemMetric>> {
    let metrics = sqlx::query_as!(
        SystemMetric, "SELECT 
        host_id, 
        cpu_usage, 
        ram_usage_mb, 
        disk_usage_percent,
        net_rx_kb,
        net_tx_kb,
        gpu_usage,
        gpu_temp,
        gpu_vram_used_mb,
        timestamp 
        FROM metrics ORDER BY timestamp DESC LIMIT 50"
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(metrics)
}

pub async fn stream_metrics(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    println!("New Browser Connected!");
    let mut rx = state.tx.subscribe();
    
    let stream = async_stream::stream! {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                yield Ok(Event::default().data(json));
            }
        }
    };

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

pub async fn serve_css() -> impl IntoResponse {
    let css_content = match std::fs::read_to_string("static/style.css") {
        Ok(content) => content,
        Err(_) => return (StatusCode::NOT_FOUND, "CSS not found").into_response(),
    };

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());

    (StatusCode::OK, headers, css_content).into_response()
}

pub async fn render_contact(State(state): State<AppState>) -> Html<String> {
    let context = tera::Context::new();
    let rendered = state.templates.render("contact.html", &context).unwrap();
    Html(rendered)
}

pub async fn submit_contact(
    State(state): State<AppState>,
    Form(form): Form<ContactForm>,
) -> Html<String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Save to DB
    let _ = sqlx::query!(
        "INSERT INTO messages (name, email, content, submitted_at) VALUES ($1, $2, $3, $4)",
        form.name, form.email, form.message, timestamp
    )
    .execute(&state.pool)
    .await;

    // Send Email Notification
    let email_user = env::var("SMTP_USERNAME").unwrap_or_default();
    let email_pass = env::var("SMTP_PASSWORD").unwrap_or_default();
    let dest_email = env::var("DESTINATION_EMAIL").unwrap_or_default();

    if !email_user.is_empty() && !email_pass.is_empty() {
        // Construct the email
        let email = Message::builder()
            .from(format!("Portfolio Bot <{}>", email_user).parse().unwrap())
            .to(dest_email.parse().unwrap())
            .subject(format!("New Contact: {}", form.name))
            .body(format!(
                "Name: {}\nEmail: {}\n\nMessage:\n{}",
                form.name, form.email, form.message
            ))
            .unwrap();

        // Connect to Gmail (or other SMTP)
        let creds = Credentials::new(email_user, email_pass);
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send!
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => println!("Could not send email: {:?}", e),
        }
    }


    // Render a "Thank You" page (or reuse contact with a success flag)
    let mut context = tera::Context::new();
    context.insert("success", &true);
    let rendered = state.templates.render("contact.html", &context).unwrap();
    Html(rendered)
}

// AI HANDLERS
pub async fn seed_knowledge(
    State(state): State<AppState>,
    Json(payload): Json<SeedRequest>,
) -> &'static str {

    for fact in payload.facts {
        let embedding = state.brain.embed(&fact).await;
        let vector = Vector::from(embedding);

        sqlx::query!(
            "INSERT INTO documents (content, embedding) VALUES ($1, $2)",
            fact,
            vector as Vector
        )
        .execute(&state.pool)
        .await
        .unwrap();
    }

    "Knowledge Base Updated"
}

pub async fn chat_handler(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> String {
    let question_vector = state.brain.embed(&payload.question).await;
    let vector_pg = Vector::from(question_vector);

    let rows = sqlx::query!(
        "SELECT content FROM documents ORDER BY embedding <-> $1 LIMIT 3",
        vector_pg as Vector
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    let context = rows.iter()
        .map(|r| r.content.clone())
        .collect::<Vec<String>>()
        .join("\n---\n");

    state.brain.ask_gemini(context, payload.question).await
}