-- Payout system tables

-- Miner balances (per coin)
CREATE TABLE IF NOT EXISTS balances (
    id SERIAL PRIMARY KEY,
    miner_id INTEGER NOT NULL REFERENCES miners(id) ON DELETE CASCADE,
    coin VARCHAR(10) NOT NULL,
    balance DECIMAL(20, 8) NOT NULL DEFAULT 0,
    pending_balance DECIMAL(20, 8) NOT NULL DEFAULT 0,
    total_paid DECIMAL(20, 8) NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(miner_id, coin)
);

-- Payout transactions
CREATE TABLE IF NOT EXISTS payouts (
    id BIGSERIAL PRIMARY KEY,
    miner_id INTEGER NOT NULL REFERENCES miners(id) ON DELETE CASCADE,
    coin VARCHAR(10) NOT NULL,
    amount DECIMAL(20, 8) NOT NULL,
    tx_hash VARCHAR(128),
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, processing, completed, failed
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP,
    error_message TEXT
);

-- Payout settings per miner
CREATE TABLE IF NOT EXISTS payout_settings (
    id SERIAL PRIMARY KEY,
    miner_id INTEGER NOT NULL REFERENCES miners(id) ON DELETE CASCADE UNIQUE,
    min_payout_threshold DECIMAL(20, 8) NOT NULL DEFAULT 0.01,
    payout_coin VARCHAR(10) NOT NULL DEFAULT 'BTC', -- Target coin for payouts
    auto_exchange BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_balances_miner_id ON balances(miner_id);
CREATE INDEX IF NOT EXISTS idx_payouts_miner_id ON payouts(miner_id);
CREATE INDEX IF NOT EXISTS idx_payouts_status ON payouts(status);
CREATE INDEX IF NOT EXISTS idx_payouts_created_at ON payouts(created_at);

-- Trigger to update balance updated_at
CREATE OR REPLACE FUNCTION update_balance_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER balance_update_timestamp
    BEFORE UPDATE ON balances
    FOR EACH ROW
    EXECUTE FUNCTION update_balance_timestamp();
