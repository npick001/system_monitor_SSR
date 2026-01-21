CREATE TABLE IF NOT EXISTS messages (
    id SERIAL PRIMARY KEY, 
    name TEXT NOT NULL, 
    email TEXT NOT NULL, 
    content TEXT NOT NULL, 
    submitted_at BIGINT NOT NULL
);