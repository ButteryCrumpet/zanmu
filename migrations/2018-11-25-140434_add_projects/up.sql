CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

ALTER TABLE todos ADD COLUMN project_id INT;
ALTER TABLE todos ADD CONSTRAINT fk_project FOREIGN KEY (project_id) REFERENCES projects(id);