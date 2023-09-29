-- Add migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS authors (
    id          INTEGER PRIMARY KEY NOT NULL,
    name        VARCHAR(250) NOT NULL,
    surname     VARCHAR(250) NOT NULL,
    photo       BLOB,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS posts (
    id          INTEGER PRIMARY KEY NOT NULL,
    title       VARCHAR(250) NOT NULL,
    content     VARCHAR(250) NOT NULL,
    author_id   INTEGER NOT NULL,
    uploaded_file BLOB,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at  DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (author_id) REFERENCES authors(id)
);

CREATE TABLE IF NOT EXISTS users (
    id          INTEGER PRIMARY KEY NOT NULL,
    username    VARCHAR(250) NOT NULL,
    password    VARCHAR(250) NOT NULL
);