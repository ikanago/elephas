CREATE TABLE IF NOT EXISTS "users" (
    "id" VARCHAR(64) PRIMARY KEY,
    "name" VARCHAR(256) NOT NULL
);

CREATE TABLE IF NOT EXISTS "user_key_pair" (
    "user_id" VARCHAR(64) PRIMARY KEY,
    "private_key" VARCHAR(4096) NOT NULL,
    "public_key" VARCHAR(4096) NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id)
);
