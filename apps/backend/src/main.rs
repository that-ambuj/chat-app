use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/hello")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[derive(serde::Serialize)]
pub struct Data {
    pub name: String,
    pub age: i32,
}

#[get("/json")]
async fn json() -> impl Responder {
    web::Json(Data {
        name: "hello".into(),
        age: 20,
    })
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world).service(json);
    };

    Ok(config.into())
}
