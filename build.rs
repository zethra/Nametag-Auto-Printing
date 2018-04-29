extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main() {
    includedir_codegen::start("DB_SETUP")
        .dir("db_setup", Compression::Gzip)
        .build("db_setup.rs")
        .unwrap();
}
