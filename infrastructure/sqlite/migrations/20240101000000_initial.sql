-- Initial migration: verify migration pipeline is functional
CREATE TABLE IF NOT EXISTS system_info (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    key     TEXT    NOT NULL UNIQUE,
    value   TEXT    NOT NULL
);

INSERT OR IGNORE INTO system_info (key, value) VALUES ('schema_version', '1');
