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

CREATE TABLE IF NOT EXISTS "follows" (
    "follow_from_name" TEXT NOT NULL,
    "follow_to_name" TEXT NOT NULL,

    PRIMARY KEY (follow_from_name, follow_to_name),
    FOREIGN KEY (follow_from_name) REFERENCES users(name) ON DELETE CASCADE,
    FOREIGN KEY (follow_to_name) REFERENCES users(name) ON DELETE CASCADE
)
