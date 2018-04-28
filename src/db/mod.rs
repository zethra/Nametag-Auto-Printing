pub mod schema;
pub mod models;

use diesel::prelude::*;
// use std::env;
use self::models::*;
use std::fs::File;
use std::path::Path;

pub fn establish_connection() -> SqliteConnection {
    let db_file_path = Path::new("nametag.db");
    let setup = !db_file_path.exists();
    if setup {
        File::create(&db_file_path).unwrap();
    }
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let database_url = "./nametag.db";
    let database_url = db_file_path.to_str().unwrap();
    let conn = SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));
    let setup_query = include_str!("../../sql/setup.sql");
    ::diesel::sql_query(setup_query)
        .execute(&conn)
        .expect("Fail to init db");
    conn
}

pub fn new_printer(conn: &SqliteConnection, name: &str, color: &str, ip: &str, api_key: &str, slic3r_conf: &str) {
    use self::schema;
    use diesel;
    
    let new_printer = NewPrinter {
        name: name,
        status: "offline",
        color: color,
        ip: ip,
        api_key: api_key,
        slic3r_conf: slic3r_conf
    };

    diesel::insert_into(schema::printers::table)
        .values(new_printer)
        .execute(conn)
        .expect("Error saving printer");
}

pub fn get_printers(conn: &SqliteConnection) -> Vec<Printer> {
    schema::printers::dsl::printers.load::<Printer>(conn).expect("Error loading printers")
}
