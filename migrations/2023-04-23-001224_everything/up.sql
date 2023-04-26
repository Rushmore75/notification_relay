CREATE TABLE account (
    id              SERIAL PRIMARY KEY,
    email           VARCHAR UNIQUE NOT NULL,
    password_hash   BYTEA NOT NULL
);

CREATE TABLE message (
    id      SERIAL PRIMARY KEY,
    author  INT REFERENCES account (id) NOT NULL,
    date    TIMESTAMP NOT NULL DEFAULT NOW(),
    content VARCHAR NOT NULL
);

CREATE TABLE read (
    id      SERIAL PRIMARY KEY,
    account INT REFERENCES account (id) NOT NULL,
    message INT REFERENCES message (id) NOT NULL
)

