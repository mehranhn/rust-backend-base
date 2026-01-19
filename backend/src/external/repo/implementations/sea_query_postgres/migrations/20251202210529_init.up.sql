-- Add up migration script here
CREATE TYPE roles AS ENUM ('ADMIN', 'SALESMEN');

CREATE TABLE IF NOT EXISTS users(
    id uuid PRIMARY KEY,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at timestamp,
    role roles NOT NULL,
    username varchar(255) NOT NULL UNIQUE,
    hashed_password bytea NOT NULL,
    test_account_exp_in_days bigint NOT NULL DEFAULT 1,
    test_account_rx_tx_limit bigint NOT NULL DEFAULT 1000000,
	delete_inactive_customers_after_days bigint
);

CREATE TABLE IF NOT EXISTS sessions(
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at timestamp,
	expire_at timestamp,
	last_access timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);
