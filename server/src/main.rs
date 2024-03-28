use actix::{Actor, Addr};
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actors::{lobby::Lobby, ws_conn::WsConn};

mod actors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let lobby_addr = Lobby::new().start();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(lobby_addr.clone()))
            .service(web::resource("ws/").route(web::get().to(connect_client)))
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
    lobby_addr: web::Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    println!("Connecting!");
    let lobby_addr = lobby_addr.get_ref().clone();
    let client = WsConn::new(lobby_addr);
    ws::start(client, &req, stream)
}
