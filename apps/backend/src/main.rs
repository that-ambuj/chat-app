use actix_web::{
    get,
    web::{self, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;
use tokio::task::spawn_local;

mod chat_server;
mod handler;

use chat_server::{ChatServer, ChatServerHandle};
use handler::chat_ws;

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let (chat_server, cmd_handle) = ChatServer::new();

    tokio::spawn(chat_server.run());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world)
            .service(ws_handler)
            .app_data(web::Data::new(cmd_handle));
    };

    Ok(config.into())
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/ws")]
async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    cmd_handle: web::Data<ChatServerHandle>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
    spawn_local(chat_ws(session, msg_stream, (**cmd_handle).clone()));

    Ok(res)
}
