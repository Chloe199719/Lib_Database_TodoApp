-- Your SQL goes here
CREATE TABLE IF NOT EXISTS email_verify_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    token VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
