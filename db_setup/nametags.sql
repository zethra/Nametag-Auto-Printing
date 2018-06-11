CREATE TABLE IF NOT EXISTS nametags (
    nametag_id INTEGER NOT NULL PRIMARY KEY,
    nametag_name VARCHAR NOT NULL,
    nametag_printer_id INTEGER,
    nametag_comments VARCHAR
);
