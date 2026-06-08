CREATE TABLE IF NOT EXISTS app_schema.todos (
    unid UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name CHARACTER VARYING(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_todos_name ON app_schema.todos (name);
CREATE INDEX IF NOT EXISTS idx_todos_created_at ON app_schema.todos (created_at);
CREATE INDEX IF NOT EXISTS idx_todos_updated_at ON app_schema.todos (updated_at);



-- Insert data into todos table
INSERT INTO app_schema.todos (name) VALUES
('Laptop Computer'),
('Coffee Mug'),
('Smartphone'),
('Houseplant'),
('Notebook');