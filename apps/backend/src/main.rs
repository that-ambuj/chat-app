use actix_files as fs;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;
use std::path::PathBuf;
use tokio::task::spawn_local;

mod chat_server;
mod handler;

use chat_server::{ChatServer, ChatServerHandle};
use handler::chat_ws;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let (chat_server, cmd_handle) = ChatServer::new();
    tokio::spawn(chat_server.run());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(cmd_handle))
            .service(ws_handler)
            .service(fs::Files::new("", static_folder).index_file("index.html"));
    };

    Ok(config.into())
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
