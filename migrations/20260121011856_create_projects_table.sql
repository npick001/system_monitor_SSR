CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    link TEXT,
    created_at BIGINT NOT NULL
);

-- Seed some dummy data so we see something immediately
INSERT INTO projects (title, description, link, created_at)
VALUES 
('Portfolio Site', 'Server-Side Rendered portfolio using Tera templates and Docker.', 'https://github.com/npick001/system_monitor_SSR', 1700000005);
INSERT INTO projects (title, description, link, created_at) VALUES 
(
    'HPC Distributed CNN Pipeline', 
    'A scalable Deep Learning training infrastructure built with Python and PyTorch. Deployed on SLURM-managed HPC clusters to analyze strong vs. weak scalability characteristics across massive parallelism levels. Optimized gradient synchronization for sub-second latency.', 
    'https://github.com/npick001/CNN-Training', 
    1735700000 
),
(
    'C++ Game AI Suite', 
    'A collection of autonomous agents including a Connect 4 bot utilizing Minimax with Alpha-Beta pruning and a Pukoban solver using heuristic search algorithms. Demonstrates low-level memory management and algorithmic optimization in C++.', 
    'https://github.com/npick001/Connect4-Solver', 
    1704100000
),
(
    'Constraint Satisfaction Engine', 
    'A high-performance CSP solver designed for complex combinatorial problems like crosswords. Implements Forward Checking, Backtracking, and custom data structures for efficient constraint propagation and domain reduction.', 
    'https://github.com/npick001/CrosswordSolver', 
    1712000000
);