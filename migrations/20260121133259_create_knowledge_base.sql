-- 1. Enable the extension
CREATE EXTENSION IF NOT EXISTS vector;

-- 2. Create the table
-- We use 384 dimensions because that is what the standard "all-MiniLM-L6-v2" model uses.
CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    embedding vector(384)
);