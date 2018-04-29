pub mod schema;
pub mod models;

use diesel::prelude::*;
use self::models::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;

include!(concat!(env!("OUT_DIR"), "/db_setup.rs"));

pub fn init() {
    let db_file_path = Path::new("nametag.db");
    let setup = !db_file_path.exists();
    if setup {
        File::create(&db_file_path).unwrap();
    }
    let database_url = db_file_path.to_str().unwrap();
    let conn = SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));
    for file_name in DB_SETUP.file_names() {
        let mut file = DB_SETUP.read(file_name).unwrap();
        let mut query = String::new();
        file.read_to_string(&mut query).unwrap();
        ::diesel::sql_query(query)
            .execute(&conn)
            .expect("Fail to init db");
    }
    // ::diesel::sql_query(setup_query)
    //     .execute(&conn)
    //     .expect("Fail to init db");
}


pub fn establish_connection() -> SqliteConnection {
    let database_url = "nametag.db";
    SqliteConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
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

pub fn new_nametag(conn: &SqliteConnection, name: &str, comments: Option<&str>) {
    use self::schema;
    use diesel;

    let new_nametag = NewNametag {
        name: name,
        comments: comments
    };

    diesel::insert_into(schema::nametags::table)
        .values(new_nametag)
        .execute(conn)
        .expect("Error saving nametag");
}

pub fn get_nametags(conn: &SqliteConnection) -> Vec<Nametag> {
    schema::nametags::dsl::nametags.load::<Nametag>(conn).expect("Error loading nametags")
}
