CREATE TABLE IF NOT EXISTS favorites (
    id TEXT NOT NULL PRIMARY KEY,
    pattern_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    FOREIGN KEY(pattern_id) REFERENCES patterns(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS patterns (
    id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    homepage_url TEXT NOT NULL,
    highlight_image_url TEXT,
    created_at DATETIME NOT NULL,
    times_visited_in_24_hours INTEGER NOT NULL,
    num_favorites INTEGER NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS users (
    id TEXT NOT NULL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);