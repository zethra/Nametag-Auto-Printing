use super::schema::printers;
use super::schema::PrinterState;

#[derive(Queryable, Serialize, Debug)]
pub struct Printer {
    pub id: i32,
    pub name: String,
    pub status: PrinterState,
    pub nametag_id: Option<i32>,
    pub color: String,
    pub ip: String,
    pub api_key: String,
    pub slic3r_conf: String
}

#[derive(Insertable, Debug)]
#[table_name = "printers"]
pub struct NewPrinter {
    pub name: String,
    pub status: PrinterState,
    pub color: String,
    pub ip: String,
    pub api_key: String,
    pub slic3r_conf: String
}

use super::schema::nametags;

#[derive(Queryable, Serialize)]
pub struct Nametag {
    pub id: i32,
    pub name: String,
    pub comments: Option<String>
}

#[derive(Insertable)]
#[table_name = "nametags"]
pub struct NewNametag {
    pub name: String,
    pub comments: Option<String>
}
