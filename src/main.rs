use axum::routing::post;
use serde::{Deserialize, Serialize};
use axum::extract::{ Json, State, FromRequest };
use socketioxide::{ extract::{ AckSender, Data, SocketRef }, SocketIo };
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
// use serde_json::{json, Value};

#[derive(Deserialize,Serialize, Debug)]
struct IgniteReq{
    group_id:String,
    event_name:String,
    message:String
}

#[derive(Debug, Deserialize, Serialize)]
struct Auth {
    token:String
}

fn req_maker( io:SocketIo, req_body: Json<IgniteReq>) {
    let _ = io
        // .to("room1")
        .emit("MESSAGE", IgniteReq{
        group_id:"1q2w3e4r5t6y7u8xcfvgb".to_owned(),
        event_name:"event from handler".to_owned(),
        message:format!("This message was triggered beacuse the someone made a GET request to the handler server with path: {:?}", req_body)
    });
}

fn on_connect(socket: SocketRef){
    println!("Socket {:?} connected !!", socket.id); 

    let _ = socket.emit("OK", "THIS IS THE SERVER TALKING TO YOU ✌️🫂");

    let _ = socket.on("JOIN", |socket_ref: SocketRef, Data(room):Data<String>, ack:AckSender| async move { 
        println!("CLIENT MADE A REQUEST TO JOIN ROOM: {}", room);
        socket_ref.join(room);
        let _ = ack.send("Room Joined");
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();
    
    io.ns("/", |socket: SocketRef, Data(auth): Data<Auth>| async move { 
        // we need to do a DB query to check if token correct
        if auth.token.is_empty() {
            println!("Invalid token, disconnecting");
            socket.emit("ERROR", "Invalid API KEY, Cant establish connection with the server !");
            socket.disconnect().ok();
            return;
        }
        on_connect(socket) 
    });

    let shared_state = io.clone();

    let app = axum::Router::new()
        .route("/ignite", post(|body: Json<IgniteReq>| async { req_maker(shared_state, body); }))
        .with_state(io)
        .layer( ServiceBuilder::new().layer(CorsLayer::permissive()).layer(layer) )
    ;

    info!("Starting server");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}