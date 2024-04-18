-- Add migration script here
CREATE TABLE IF NOT EXISTS addresses
(
    id INTEGER PRIMARY KEY NOT NULL,
    address TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT '',
    pnl_1d REAL NOT NULL DEFAULT 0.00,
    pnl_7d REAL NOT NULL DEFAULT 0.00,
    pnl_30d REAL NOT NULL DEFAULT 0.00,
    realized_profit REAL NOT NULL DEFAULT 0.00,
    realized_profit_7d REAL NOT NULL DEFAULT 0.00,
    last_active INTEGER NOT NULL DEFAULT 0,
    created INTEGER NOT NULL DEFAULT 0,
    updated INTEGER NOT NULL DEFAULT 0,
    deleted INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS tokens
(
    id INTEGER PRIMARY KEY NOT NULL,
    owner TEXT NOT NULL,
    token_address TEXT NOT NULL,
    token_name TEXT NOT NULL,
    token_symbol TEXT NOT NULL,
    token_icon TEXT NOT NULL DEFAULT '',
    amount TEXT NOT NULL,
    price_usdt REAL NOT NULL DEFAULT 0.00,
    created INTEGER NOT NULL DEFAULT 0,
    updated INTEGER NOT NULL DEFAULT 0,
    deleted INTEGER NOT NULL DEFAULT 0
);
