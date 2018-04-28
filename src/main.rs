extern crate nap;
extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate env_logger;
extern crate serde_json;
extern crate diesel;
#[macro_use] extern crate serde_derive;

use actix_web::*;
use actix_web::http::Method;
use diesel::prelude::*;
use nap::nametag;
use nap::db;
use std::fs::File;
use std::io::Read;

fn index(_: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/index.html")?)
}

fn preview(req: HttpRequest) -> Result<HttpResponse> {
    let name = match req.query().get("name") {
        Some(name) => name,
        None => {
            return Ok(HttpResponse::BadRequest().body("Must include name param"));
        }
    };
    if *req.method() == Method::HEAD {
        return Ok(HttpResponse::Ok().body(""));
    }
    if *req.method() != Method::GET {
        return Ok(HttpResponse::NotImplemented().body("Method not supported"));
    }
    // println!("{:?}", name);
    nametag::preview(&name)?;
    let file = format!("./{}.png", name);
    println!("{}", file);
    let mut file = File::open(&file)?;
    let mut b = Vec::new();
    file.read_to_end(&mut b).unwrap();
    Ok(HttpResponse::Ok().body(b))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();

    let conn = db::establish_connection();
    db::new_printer(&conn, "test2", "green", "127.0.0.1", "api", "conf");

    let res = db::get_printers(&conn);
    println!("Displaying {} printers", res.len());
    for printer in res {
        println!("{}", printer.name);
    }

    return;

    let sys = actix::System::new("json-example");
    let _addr = server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            // .resource("/", |r| r.method(Method::POST).f(index))})
            .handler("/static/", fs::StaticFiles::new("./static/"))
            .resource("/", |r| r.method(Method::GET).f(index))
            .resource("/preview", |r| r.f(preview))
        })
        .bind("127.0.0.1:8080").unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
