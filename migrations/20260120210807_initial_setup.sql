CREATE TABLE IF NOT EXISTS metrics (
    id SERIAL PRIMARY KEY,
    host_id TEXT NOT NULL,
    cpu_usage REAL NOT NULL,
    ram_usage_mb REAL NOT NULL,
    disk_usage_percent REAL NOT NULL,
    net_rx_kb REAL NOT NULL,
    net_tx_kb REAL NOT NULL,
    timestamp BIGINT NOT NULL
);