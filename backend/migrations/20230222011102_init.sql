CREATE TABLE IF NOT EXISTS "users" (
    "name" TEXT PRIMARY KEY,
    "password_hash" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "user_key_pair" (
    "user_name" TEXT PRIMARY KEY,
    "private_key" TEXT NOT NULL,
    "public_key" TEXT NOT NULL,

    FOREIGN KEY (user_name) REFERENCES users(name) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "posts" (
    "id" TEXT PRIMARY KEY,
    "user_name" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "published_at" TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (user_name) REFERENCES users(name) ON DELETE CASCADE
);
