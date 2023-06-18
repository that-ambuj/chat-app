use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    get,
    web::{self, Data, ServiceConfig},
    Error, HttpRequest, HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;
use tokio::sync::mpsc;

mod handler;
mod chat_server;

use handler::chat_ws;
use tokio::task::spawn_local;

pub struct AppState {
    pub sessions: Mutex<HashMap<usize, mpsc::UnboundedSender<String>>>,
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_state = Data::new(AppState {
        sessions: Mutex::new(HashMap::new()),
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.configure(route_config).app_data(app_state);
    };

    Ok(config.into())
}

fn route_config(cfg: &mut ServiceConfig) {
    cfg.service(hello_world).service(ws_handler);
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/ws")]
async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
    spawn_local(chat_ws(session, msg_stream, state.into_inner()));

    Ok(res)
}
