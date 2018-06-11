extern crate diesel;
extern crate nap;

use diesel::prelude::*;
use nap::db::models::*;
use nap::db::schema::PrinterState;

fn main() {
    use nap::db::schema::nametags::dsl::*;
    use nap::db::schema::printers::dsl::*;
    let conn = SqliteConnection::establish("nametag.db").unwrap();
    // let ret = nametags.inner_join(printers::table).load::<Vec<(Nametag, Printer)>>(&conn).unwrap();
    let ret = nametags.load::<Nametag>(&conn).unwrap();
    println!("{:?}", ret);
}
