#[derive(Debug, DbEnum, Serialize)]
pub enum PrinterState {
    Idle,
    Printing,
    Done,
    Unavailable
}

table! {
    use diesel::sql_types::*;
    use super::PrinterStateMapping;
    printers (printer_id) {
        printer_id -> Integer,
        printer_name -> Text,
        printer_status -> PrinterStateMapping,
        printer_color -> Text,
        printer_ip -> Text,
        printer_api_key -> Text,
        printer_slic3r_conf -> Text,
    }
}

table! {
    nametags (nametag_id) {
        nametag_id -> Integer,
        nametag_name -> Text,
        nametag_printer_id -> Nullable<Integer>,
        nametag_comments -> Nullable<Text>,
    }
}
