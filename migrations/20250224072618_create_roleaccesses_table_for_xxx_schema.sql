CREATE TYPE user_role AS ENUM ('Root', 'Admin', 'RegularUser');

CREATE TABLE IF NOT EXISTS app_schema.roleaccesses
(
    unid                          UUID PRIMARY KEY               NOT NULL,
    created                       TIMESTAMPTZ                    NOT NULL,
    role                          user_role                      NOT NULL DEFAULT 'RegularUser',
    grantedto_unid                UUID                           NOT NULL,
    FOREIGN KEY (grantedto_unid)  REFERENCES app_schema.users (unid)
);

CREATE INDEX IF NOT EXISTS idxdelv9fhux3r0nta48kcksu4rg on app_schema.roleaccesses using btree (grantedto_unid);
CREATE INDEX IF NOT EXISTS idxk2545uom1an1a2akehb554c91 on app_schema.roleaccesses using btree (role);




INSERT INTO app_schema.roleaccesses (
    unid,
    created,
    role,
    grantedto_unid
) VALUES 
-- ROOT Lastname - Root user
(
    'c3d4e5f6-a789-0123-cdef-456789012345',
    now(),
    'Root',
    '485d1ad0-792c-436b-a790-17c106135c67'
),
-- Alice Aubert - RegularUser
(
    'b8c9d0e1-f234-5678-bcde-9012345bcdef',
    now(),
    'RegularUser',
    '7e3d2c1b-8a9f-4e5d-b6c7-1f2e3d4c5b6a'
),
-- Bob Baker - RegularUser
(
    'd0e1f2a3-b456-789a-def0-12345bcdef01',
    now(),
    'RegularUser',
    '4b6e8d2c-1a3f-4c7b-9e0d-2f5a6c7e8b9d'
),
-- Admin Stuff - Admin
(
    'f2a3b4c5-d678-9abc-ef01-2345bcdef013',
    now(),
    'Admin',
    '9f8e7d6c-5b4a-3210-fedc-ba9876543210'
)
ON CONFLICT (unid) DO NOTHING;