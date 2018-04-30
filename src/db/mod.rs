pub mod schema;
pub mod models;

use actix_web::*;
use actix::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use self::models::*;
use std::fs::File;
use std::path::Path;
use std::io::Read;

include!(concat!(env!("OUT_DIR"), "/db_setup.rs"));

pub struct DbExecutor(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

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


pub fn establish_connection() -> ConnectionManager<SqliteConnection> {
    let database_url = "nametag.db";
    ConnectionManager::<SqliteConnection>::new(database_url)
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

impl Message for NewNametag {
    type Result = Result<(), Error>;
}

impl Handler<NewNametag> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: NewNametag, _: &mut Self::Context) -> Self::Result {
        use diesel;

        let conn: &SqliteConnection = &self.0.get().unwrap();
        diesel::insert_into(schema::nametags::table)
            .values(msg)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error instering nametag"))?;
        Ok(())
    }
}

pub struct GetNametags;

impl Message for GetNametags {
    type Result = Result<Vec<Nametag>, Error>;
}

impl Handler<GetNametags> for DbExecutor {
    type Result = Result<Vec<Nametag>, Error>;

    fn handle(&mut self, _:GetNametags, _: &mut Self::Context) -> Self::Result {
        let conn: &SqliteConnection = &self.0.get().unwrap();
        let ret = schema::nametags::dsl::nametags.load::<Nametag>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading printers"))?;
        Ok(ret)
    }
}
    

// pub fn new_nametag(conn: &SqliteConnection, name: &str, comments: Option<&str>) {
//     use self::schema;
//     use diesel;
//
//     let new_nametag = NewNametag {
//         name: name,
//         comments: comments
//     };
//
// }

// pub fn get_nametags(conn: &SqliteConnection) -> Vec<Nametag> {
//     schema::nametags::dsl::nametags.load::<Nametag>(conn).expect("Error loading nametags")
// }
