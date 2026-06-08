CREATE TABLE IF NOT EXISTS app_schema.bug_reports (
    unid        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bug_type    VARCHAR(50)  NOT NULL,
    message     TEXT         NOT NULL,
    user_login  VARCHAR(255),
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT now()
);
