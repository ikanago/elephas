CREATE TABLE IF NOT EXISTS "users" (
    "id" TEXT PRIMARY KEY,
    "name" TEXT NOT NULL,
    "password_hash" TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS "user_key_pair" (
    "user_id" TEXT PRIMARY KEY,
    "private_key" TEXT NOT NULL,
    "public_key" TEXT NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS "posts" (
    "id" TEXT PRIMARY KEY,
    "user_id" TEXT NOT NULL,
    "content" TEXT NOT NULL,
    "published_at" TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
