CREATE TABLE IF NOT EXISTS "users" (
    "id" SERIAL PRIMARY KEY,
    "email" VARCHAR(256) NOT NULL,
    "name" VARCHAR(256) NOT NULL
);

CREATE TABLE IF NOT EXISTS "user_key_pair" (
    "user_id" INT PRIMARY KEY,
    "private_key" VARCHAR(4096) NOT NULL,
    "public_key" VARCHAR(4096) NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id)
);
