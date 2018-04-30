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
    printers (id) {
        id -> Integer,
        name -> Text,
        status -> PrinterStateMapping,
        nametag_id -> Nullable<Integer>,
        color -> Text,
        ip -> Text,
        api_key -> Text,
        slic3r_conf -> Text,
    }
}

table! {
    nametags (id) {
        id -> Integer,
        name -> Text,
        comments -> Nullable<Text>,
    }
}
