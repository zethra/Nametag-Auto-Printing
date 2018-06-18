use super::db::models::*;
use super::db::DbExecutor;
use actix::prelude::*;
use actix_web::*;

struct PrinterDaemon {
    db: Addr<Syn, DbExecutor>
}

impl Actor for PrinterDaemon {
    type Context = Context<Self>;
}

impl Handler<NewNametag> for PrinterDaemon {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: NewNametag, _: &mut Self::Context) -> Self::Result {
        Ok(())
    }
}
