extern crate nap;
extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate futures;
extern crate env_logger;
extern crate diesel;
#[macro_use] 
extern crate serde_derive;
extern crate r2d2;

use actix::prelude::*;
use actix_web::*;
use actix_web::http::Method;
use futures::future::Future;
use nap::nametag;
use nap::db::*;
use nap::db::DbExecutor;
use nap::db::models::*;
use std::fs::File;
use std::io::Read;

struct AppState {
    db: Addr<Syn, DbExecutor>
}

fn preview(req: HttpRequest<AppState>) -> Result<HttpResponse> {
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

fn submit(form: Form<SubmitForm>, state: State<AppState>) -> FutureResponse<HttpResponse> {
    state.db.send(NewNametag{name:form.name.clone(), comments: None})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(_) => Ok(HttpResponse::Ok().into()),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

fn list_nametags(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(GetNametags{})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(res) => Ok(HttpResponse::Ok().json(res)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

fn list_printers(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(GetPrinters{})
        .from_err()
        .and_then(|res| {
            match res {
                Ok(res) => Ok(HttpResponse::Ok().json(res)),
                Err(_) => Ok(HttpResponse::InternalServerError().into())
            }
        }).responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();

    init();

    let sys = actix::System::new("nametag-auto-printing");

    // Create conncection pool w 3 db connections
    let manager = establish_connection();
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");
    let addr = SyncArbiter::start(3, move || {
        DbExecutor(pool.clone())
    });
    
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .handler("/", fs::StaticFiles::new("./static/").index_file("index.html"))
            .resource("/preview", |r| r.f(preview))
            .resource("/submit", |r| r.method(Method::POST).with2(submit))
            .resource("/nametags", |r| r.f(list_nametags))
            .resource("/printers", |r| r.f(list_printers))
        })
        .bind("127.0.0.1:8080").unwrap()
        .shutdown_timeout(1)
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
