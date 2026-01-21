use crate::state::AppState;
use pgvector::Vector;
use std::fs;

pub async fn auto_seed(state: &AppState) {
    let content = match fs::read_to_string("knowledge.txt") {
        Ok(c) => c,
        Err(_) => {
            println!("⚠️ No knowledge.txt found. Skipping auto-seed.");
            return;
        }
    };

    // OPTIONAL: Wipe old data to ensure the 'injection' is fresh
    let _ = sqlx::query("DELETE FROM documents")
        .execute(&state.pool)
        .await;

    println!("🧠 Seeding facts from knowledge.txt into FastEmbed...");

    for line in content.lines() {
        let fact = line.trim();
        if fact.is_empty() || fact.starts_with('#') { continue; }

        let embedding = state.brain.embed(fact).await;
        let vector = Vector::from(embedding);

        sqlx::query!(
            "INSERT INTO documents (content, embedding) VALUES ($1, $2)",
            fact,
            vector as Vector
        )
        .execute(&state.pool)
        .await
        .expect("Failed to insert fact");
    }

    println!("✅ Brain is now synchronized with knowledge.txt");
}