CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    priority BOOLEAN NOT NULL,
    state VARCHAR(10) NOT NULL DEFAULT 'active'
)