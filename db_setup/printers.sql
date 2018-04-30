CREATE TABLE IF NOT EXISTS printers (
    id INTEGER NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL,
    status VARCHAR CHECK(status IN ('idle', 'printing', 'done', 'unavailable')) NOT NULL,
    nametag_id INTEGER,
    color VARCHAR NOT NULL,
    api_key VARCHAR NOT NULL,
    ip VARCHAR NOT NULL,
    slic3r_conf VARCHAR NOT NULL
);

