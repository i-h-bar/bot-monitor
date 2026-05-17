CREATE TABLE register (
    bot_id        TEXT NOT NULL,
    user_id       TEXT NOT NULL,
    entry_version TEXT NOT NULL,
    PRIMARY KEY (bot_id, user_id)
);

CREATE INDEX register_user_id_idx ON register (user_id);
