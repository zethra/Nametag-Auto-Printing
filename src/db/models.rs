use super::schema::printers;

#[derive(Queryable)]
pub struct Printer {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub active: bool,
    pub selectable: bool,
    pub nametag_id: Option<i32>,
    pub color: String,
    pub ip: String,
    pub api_key: String,
    pub slic3r_conf: String
}

#[derive(Insertable)]
#[table_name = "printers"]
pub struct NewPrinter<'a> {
    pub name: &'a str,
    pub status: &'a str,
    pub color: &'a str,
    pub ip: &'a str,
    pub api_key: &'a str,
    pub slic3r_conf: &'a str
}
