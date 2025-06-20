
CREATE TABLE users (
    user_id      TEXT PRIMARY KEY NOT NULL,
    mitid_id     TEXT UNIQUE      NOT NULL,
    created      TEXT DEFAULT (datetime('now')),
    name         TEXT             NOT NULL,
    email_prefix TEXT UNIQUE
);

CREATE UNIQUE INDEX idx_users_mitid ON users (mitid_id);
CREATE UNIQUE INDEX idx_users_email_prefix ON users (email_prefix);

CREATE TABLE emails (
    email_id  TEXT PRIMARY KEY NOT NULL,
    user_id   TEXT             NOT NULL,
    sender    TEXT             NOT NULL,
    recipient TEXT             NOT NULL,
    date      TEXT             NOT NULL,
    subject   TEXT             NOT NULL,
    play      TEXT             NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id) 
);

CREATE INDEX idx_emails_user_id ON emails (user_id);
CREATE INDEX idx_emails_date ON emails (date);
