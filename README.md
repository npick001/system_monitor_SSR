# System Monitor SSR

A full-stack Rust application for real-time system monitoring with server-side rendering, AI-powered chat, and portfolio features. This project demonstrates a modern Rust web architecture with distributed monitoring agents, PostgreSQL with vector search, and WebSocket streaming.

## 🚀 Features

- **Real-time System Monitoring**: Track CPU, RAM, disk, network, and GPU metrics
- **Multi-Host Support**: Monitor multiple machines simultaneously with unique host IDs
- **Live Metrics Streaming**: Server-Sent Events (SSE) for real-time dashboard updates
- **AI-Powered Chat**: RAG (Retrieval-Augmented Generation) chatbot using pgvector for semantic search
- **Portfolio Integration**: Showcase projects with a modern, server-rendered website
- **Docker Ready**: Full containerization with Docker Compose
- **NVIDIA GPU Support**: Optional GPU monitoring with NVML wrapper

## 📁 Project Structure

```
system_monitor_SSR/
├── agent/              # Monitoring agent (collects system metrics)
├── backend/            # Web server and API (Axum + PostgreSQL)
├── shared/             # Shared data structures between agent and backend
├── migrations/         # SQL migrations (SQLx)
├── templates/          # Tera templates for SSR
├── static/             # CSS and JavaScript files
├── docker/             # Dockerfile configurations
└── Cargo.toml          # Workspace configuration
```

## 🛠️ Technology Stack

### Backend
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Database**: PostgreSQL with [pgvector](https://github.com/pgvector/pgvector) extension
- **ORM**: [SQLx](https://github.com/launchbadge/sqlx) with compile-time checked queries
- **Template Engine**: [Tera](https://github.com/Keats/tera)
- **AI/LLM**: Google Gemini API integration with vector embeddings
- **Email**: Lettre SMTP client for contact form

### Agent
- **System Monitoring**: [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
- **GPU Monitoring**: [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) (NVIDIA only)
- **HTTP Client**: [reqwest](https://github.com/seanmonstar/reqwest)

### Shared
- **Serialization**: [Serde](https://serde.rs/)
- **Async Runtime**: [Tokio](https://tokio.rs/)

## 📋 Prerequisites

- **Rust**: 1.70+ (install from [rustup.rs](https://rustup.rs/))
- **PostgreSQL**: 16+ with pgvector extension
- **Docker & Docker Compose** (optional, for containerized deployment)
- **NVIDIA Drivers** (optional, for GPU monitoring)

## 🔧 Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd system_monitor_SSR
```

### 2. Configure Environment

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://user:password@localhost:5432/monitor_db
OPENAI_API_KEY=your_openai_api_key_here
SMTP_USERNAME=your_email@gmail.com
SMTP_PASSWORD=your_app_password
SMTP_FROM=your_email@gmail.com
SMTP_TO=recipient@example.com
```

### 3. Set Up Database

#### Option A: Using Docker Compose (Recommended)

```bash
docker-compose up -d postgres
```

#### Option B: Manual PostgreSQL Setup

```sql
CREATE DATABASE monitor_db;
CREATE EXTENSION vector;
```

### 4. Run Migrations

```bash
cargo install sqlx-cli
sqlx migrate run
```

## 🚀 Running the Application

### Development Mode

#### Run Backend Server
```bash
cargo run -p monitor_backend
```

#### Run Monitoring Agent
```bash
cargo run -p monitor_agent
```

#### Run Both (Using VS Code Tasks)
Use the "Run ALL" task in VS Code, or run:
```bash
# Terminal 1
cargo run -p monitor_backend

# Terminal 2
cargo run -p monitor_agent
```

### Production Mode (Docker)

```bash
# Start everything
docker-compose up -d

# View logs
docker logs -f monitor_app

# Stop everything
docker-compose down
```

## 🌐 API Endpoints

### Web Pages (SSR)
- `GET /` - Portfolio home page
- `GET /monitor` - Real-time monitoring dashboard
- `GET /about` - About page
- `GET /resume` - Resume page
- `GET /contact` - Contact form

### API Routes
- `POST /ingest` - Ingest system metrics from agents
- `GET /history` - Retrieve historical metrics
- `GET /events` - SSE stream for real-time metrics
- `POST /chat` - AI chatbot endpoint
- `POST /contact` - Submit contact form
- `POST /seed` - Seed knowledge base for AI

### Static Files
- `/static/style.css` - Main stylesheet
- `/static/monitor.js` - Monitoring dashboard JavaScript
- `/static/portfolio.js` - Portfolio page JavaScript

## 📊 System Metrics

The agent collects and transmits:

- **CPU**: Global CPU usage percentage
- **RAM**: Used memory in MB
- **Disk**: Total disk usage percentage
- **Network**: Receive/transmit data in KB
- **GPU** (NVIDIA only):
  - GPU utilization percentage
  - GPU temperature (°C)
  - VRAM usage in MB

## 🤖 AI Chat Features

The backend includes an AI-powered chatbot with:
- **Vector Search**: Uses pgvector for semantic similarity matching
- **RAG Pipeline**: Retrieves relevant context from knowledge base
- **OpenAI Integration**: GPT-powered responses with context
- **Knowledge Seeding**: Add custom facts to the knowledge base

Example chat request:
```bash
curl -X POST http://localhost:3000/chat \
  -H "Content-Type: application/json" \
  -d '{"question": "What do you know about system monitoring?"}'
```

## 🐳 Docker Commands

```bash
# Build and start
docker-compose up -d --build

# View logs
docker logs -f monitor_app
docker logs -f monitor_db

# Stop services
docker-compose down

# Reset database (⚠️ deletes all data)
docker-compose down -v

# Shell into container
docker exec -it monitor_app /bin/bash

# Database shell
docker exec -it monitor_db psql -U user -d monitor_db
```

## 🗄️ Database Migrations

```bash
# Create new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Update SQLx offline query cache (required for Docker builds)
cargo sqlx prepare --workspace
```

## 📦 Project Components

### Agent (`agent/`)
Lightweight system monitoring daemon that collects metrics and streams them to the backend every second.

### Backend (`backend/`)
- **main.rs**: Server setup, routing, and middleware
- **handlers.rs**: HTTP handlers for pages and API endpoints
- **ai.rs**: RAG implementation with vector embeddings
- **state.rs**: Shared application state
- **seeder.rs**: Auto-seeding knowledge base on startup

### Shared (`shared/`)
Common data structures used by both agent and backend.

## 🔐 Security Notes

- Store sensitive credentials in `.env` (not committed to Git)
- Use environment variables for API keys and passwords
- SMTP credentials should use app-specific passwords
- Consider enabling HTTPS in production

## 🛠️ Development

### Update Dependencies

```bash
cargo update
```

### Check Code

```bash
cargo check --workspace
cargo clippy --workspace
```

### Run Tests

```bash
cargo test --workspace
```

### Format Code

```bash
cargo fmt --workspace
```

## 📝 Migrations Overview

1. **initial_setup**: Create metrics table with vector support
2. **create_projects_table**: Portfolio projects
3. **create_messages_table**: Contact form messages
4. **add_gpu_metrics**: GPU monitoring columns
5. **create_knowledge_base**: AI knowledge base with embeddings

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Powered by Axum, SQLx, and the amazing Rust ecosystem
- GPU monitoring via NVIDIA NVML
- Vector search via pgvector
