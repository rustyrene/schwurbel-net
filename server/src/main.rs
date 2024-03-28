use actix::{Actor, Addr};
use actix_files as fs;
use actix_web::{
    web::{self, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use actors::{lobby::Lobby, ws_conn::ws_conn};

mod actors;

#[derive(Clone)]
struct AppState {
    lobby_addr: Addr<Lobby>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState {
        lobby_addr: Lobby::new().start(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state))
            .service(fs::Files::new("/", "../static").index_file("index.html"))
    })
    .workers(4)
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

async fn connect_client(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let lobby_addr = &app_state.lobby_addr;
    let client = ws_conn::new(lobby_addr.clone());
    ws::start(client, &req, stream)
}
