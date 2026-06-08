CREATE TABLE IF NOT EXISTS app_schema.users
(
    unid                         UUID PRIMARY KEY               NOT NULL,
    version                      INTEGER                        NOT NULL,
    created                      TIMESTAMPTZ                    NOT NULL,
    lastpasswordchange           TIMESTAMPTZ                    NOT NULL,
    email                        CHARACTER VARYING(60)          NOT NULL,
    password                     CHARACTER VARYING(255)         NOT NULL,
    firstname                    CHARACTER VARYING(30)          NOT NULL,
    lastname                     CHARACTER VARYING(30)          NOT NULL,
    salt                         CHARACTER VARYING(16)          NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ow0gan20590jrb00upg3va2fn on app_schema.users using btree (email);
CREATE INDEX IF NOT EXISTS idx6tkmbeslcpqqeyvib5c071fit on app_schema.users using btree (firstname);
CREATE INDEX IF NOT EXISTS idxow0gan20590jrb00upg3va2fn on app_schema.users using btree (email);
CREATE INDEX IF NOT EXISTS idxpn944uhcpdq3rbcqsd0bmqq7n on app_schema.users using btree (firstname, lastname, email);
CREATE INDEX IF NOT EXISTS userloginlc on app_schema.users using btree ((TRIM(BOTH FROM lower((email)::text))));


-- ROOT
INSERT INTO app_schema.users (
    unid,
    version,
    created,
    lastpasswordchange,
    email,
    password,
    firstname,
    lastname,
    salt
) VALUES (
    '485d1ad0-792c-436b-a790-17c106135c67',
    18064,
    '2010-11-21 05:42:57.243+04',
    '2020-01-12 02:45:14.843642+04',
    'root@example.com',
    '$6$a8q/LStXr5d9WUGs$D59OuFN7SsKhUYBLGGkb1hNWiX5mEdprxPJKxFnooJ0VLHNNehYub.WRpL2OVsmVSjneUNvMnk50chhOi8Dt61',
    'Root',
    'Lastname',
    'a8q/LStXr5d9WUGs'
) ON CONFLICT (unid) DO NOTHING;


-- ADMIN
INSERT INTO app_schema.users (
    unid,
    version,
    created,
    lastpasswordchange,
    email,
    password,
    firstname,
    lastname,
    salt
) VALUES (
    '9f8e7d6c-5b4a-3210-fedc-ba9876543210',
    18064,
    '2010-11-21 05:42:57.243+04',
    '2020-01-12 02:45:14.843642+04',
    'admin@example.com',
    '$6$a8q/LStXr5d9WUGs$D59OuFN7SsKhUYBLGGkb1hNWiX5mEdprxPJKxFnooJ0VLHNNehYub.WRpL2OVsmVSjneUNvMnk50chhOi8Dt61',
    'Admin',
    'Stuff',
    'a8q/LStXr5d9WUGs'
) ON CONFLICT (unid) DO NOTHING;



-- ALICE
INSERT INTO app_schema.users (
    unid,
    version,
    created,
    lastpasswordchange,
    email,
    password,
    firstname,
    lastname,
    salt
) VALUES (
    '7e3d2c1b-8a9f-4e5d-b6c7-1f2e3d4c5b6a',
    18064,
    '2010-11-21 05:42:57.243+04',
    '2020-01-12 02:45:14.843642+04',
    'alice@example.com',
    '$6$a8q/LStXr5d9WUGs$D59OuFN7SsKhUYBLGGkb1hNWiX5mEdprxPJKxFnooJ0VLHNNehYub.WRpL2OVsmVSjneUNvMnk50chhOi8Dt61',
    'Alice',
    'Aubert',
    'a8q/LStXr5d9WUGs'
) ON CONFLICT (unid) DO NOTHING;

-- BOB
INSERT INTO app_schema.users (
    unid,
    version,
    created,
    lastpasswordchange,
    email,
    password,
    firstname,
    lastname,
    salt
) VALUES (
    '4b6e8d2c-1a3f-4c7b-9e0d-2f5a6c7e8b9d',
    18064,
    '2010-11-21 05:42:57.243+04',
    '2020-01-12 02:45:14.843642+04',
    'bob@example.com',
    '$6$a8q/LStXr5d9WUGs$D59OuFN7SsKhUYBLGGkb1hNWiX5mEdprxPJKxFnooJ0VLHNNehYub.WRpL2OVsmVSjneUNvMnk50chhOi8Dt61',
    'Bob',
    'Baker',
    'a8q/LStXr5d9WUGs'
) ON CONFLICT (unid) DO NOTHING;
