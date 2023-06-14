use actix_web::{
    get, rt,
    web::{self, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;

mod chat_server;
mod handler;

use handler::echo_ws;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    Ok(route_config.into())
}

fn route_config(cfg: &mut ServiceConfig) {
    cfg.service(hello_world).service(ws_handler);
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/ws")]
async fn ws_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    rt::spawn(echo_ws(session, msg_stream));

    Ok(res)
}
