extern crate nap;
extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate env_logger;
// extern crate serde_json;
extern crate diesel;
#[macro_use] 
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use actix_web::*;
use actix_web::http::Method;
use nap::nametag;
use nap::db;
use nap::db::models::*;
use std::fs::File;
use std::io::Read;


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
    // TODO use tmp folder
    let nt_path = match nametag::preview(&name) {
        Ok(path) => path,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().body(e));
        }
    };
    let mut file = File::open(&nt_path)?;
    let mut b = Vec::new();
    file.read_to_end(&mut b).unwrap();
    Ok(HttpResponse::Ok().body(b))
}

#[derive(Deserialize)]
struct SubmitForm {
    name: String
}

fn submit(form: Form<SubmitForm>) -> Result<HttpResponse> {
    let conn = db::establish_connection();
    db::new_nametag(&conn, &form.name, None);

    let res = db::get_nametags(&conn);
    println!("Displaying {} nametags", res.len());
    for nametag in res {
        println!("{}", nametag.name);
    }
    Ok(HttpResponse::Ok().body("<status>Success</status>"))
}

fn list_printers(_: HttpRequest) -> Result<Json<Vec<Printer>>> {
    let conn = db::establish_connection();
    let res = db::get_printers(&conn);
    Ok(Json(res))
}

fn list_nametags(_: HttpRequest) -> Result<Json<Vec<Nametag>>> {
    let conn = db::establish_connection();
    let res = db::get_nametags(&conn);
    Ok(Json(res))
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();

    db::init();

    let sys = actix::System::new("nametag-auto-printing");
    let _addr = server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .handler("/", fs::StaticFiles::new("./static/").index_file("index.html"))
            .resource("/preview", |r| r.f(preview))
            .resource("/submit", |r| r.method(Method::POST).with(submit))
            .handler("/printers", list_printers)
            .handler("/nametags", list_nametags)
        })
        .bind("127.0.0.1:8080").unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
