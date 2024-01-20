-- Your SQL goes here
CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    public_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
