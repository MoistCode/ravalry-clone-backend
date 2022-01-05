CREATE TABLE favorites (
    id TEXT NOT NULL PRIMARY KEY,
    pattern_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    FOREIGN KEY(pattern_id) REFERENCES patterns(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);