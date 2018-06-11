use super::schema::printers;
use super::schema::PrinterState;

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[table_name = "printers"]
pub struct Printer {
    pub id: i32,
    pub name: String,
    pub status: PrinterState,
    pub color: String,
    pub ip: String,
    pub key: String,
    pub slic3r_conf: String
}

#[derive(Insertable, Debug)]
#[table_name = "printers"]
pub struct NewPrinter {
    pub printer_name: String,
    pub printer_status: PrinterState,
    pub printer_color: String,
    pub printer_ip: String,
    pub printer_api_key: String,
    pub printer_slic3r_conf: String
}

use super::schema::nametags;

#[derive(Identifiable, Queryable, Associations, Serialize, PartialEq, Debug)]
#[belongs_to(Nametag, foreign_key = "nametag_printer_id")]
#[table_name = "nametags"]
pub struct Nametag {
    pub id: i32,
    pub name: String,
    pub nametag_printer_id: Option<i32>,
    pub comments: Option<String>
}

#[derive(Debug, Insertable)]
#[table_name = "nametags"]
pub struct NewNametag {
    pub nametag_name: String,
    pub nametag_comments: Option<String>
}
