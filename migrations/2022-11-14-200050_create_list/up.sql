-- Your SQL goes here
CREATE TABLE lists (
    user_hash       VARCHAR    NOT NULL PRIMARY KEY,
    list            JSONB      NOT NULL,
    updated_at      TIMESTAMP  NOT NULL DEFAULT NOW()
);