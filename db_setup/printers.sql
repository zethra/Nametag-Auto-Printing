CREATE TABLE IF NOT EXISTS printers (
    printer_id INTEGER NOT NULL PRIMARY KEY,
    printer_name VARCHAR NOT NULL,
    printer_status VARCHAR CHECK(printer_status IN ('idle', 'printing', 'done', 'unavailable')) NOT NULL,
    printer_color VARCHAR NOT NULL,
    printer_api_key VARCHAR NOT NULL,
    printer_ip VARCHAR NOT NULL,
    printer_slic3r_conf VARCHAR NOT NULL
);

