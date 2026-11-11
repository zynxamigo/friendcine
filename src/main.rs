mod m3u;
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Path, State},
    http::header,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::{atomic::{AtomicU64, Ordering}, Arc}};
use tokio::sync::{mpsc, Mutex};

type Tx = mpsc::UnboundedSender<String>;

#[derive(Default)]
struct Room {
    peers: HashMap<u64, Tx>,
    host: Option<u64>,
    host_token: String,
    catalog: Value,
    state: Value,
    sug: Vec<Value>,
    chat: Vec<Value>,
}

#[derive(Clone, Default)]
struct App {
    rooms: Arc<Mutex<HashMap<String, Room>>>,
    next: Arc<AtomicU64>,
}

fn send(tx: &Tx, v: Value) { let _ = tx.send(v.to_string()); }

fn broadcast(r: &Room, except: Option<u64>, v: Value) {
    let s = v.to_string();
    for (id, tx) in &r.peers {
        if Some(*id) != except { let _ = tx.send(s.clone()); }
    }
}

async fn ws(w: WebSocketUpgrade, Path(room): Path<String>, State(app): State<App>) -> impl IntoResponse {
    w.on_upgrade(move |s| conn(s, room, app))
}

async fn conn(sock: WebSocket, room: String, app: App) {
    let (mut sink, mut stream) = sock.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();
    let id = app.next.fetch_add(1, Ordering::Relaxed);
    tokio::spawn(async move {
        while let Some(m) = rx.recv().await {
            if sink.send(Message::Text(m.into())).await.is_err() { break; }
        }
    });
    while let Some(Ok(msg)) = stream.next().await {
        if let Message::Text(t) = msg {
            if let Ok(v) = serde_json::from_str::<Value>(&t) {
                handle(&app, &room, id, &tx, v).await;
            }
        }
    }
    let mut rooms = app.rooms.lock().await;
    if let Some(r) = rooms.get_mut(&room) {
        r.peers.remove(&id);
        if r.host == Some(id) { r.host = None; }
        broadcast(r, None, json!({"t": "left", "id": id}));
        if r.peers.is_empty() { rooms.remove(&room); }
    }
}

async fn handle(app: &App, room: &str, id: u64, tx: &Tx, v: Value) {
    let mut rooms = app.rooms.lock().await;
    let r = rooms.entry(room.to_string()).or_default();
    let is_host = r.host == Some(id);
    match v["t"].as_str().unwrap_or("") {
        "join" => {
            let tok = v["token"].as_str().unwrap_or("").to_string();
            if r.host_token.is_empty() { r.host_token = tok.clone(); }
            if r.host_token == tok { r.host = Some(id); }
            let others: Vec<u64> = r.peers.keys().cloned().collect();
            r.peers.insert(id, tx.clone());
            let host = r.host == Some(id);
            send(tx, json!({"t":"welcome","id":id,"host":host,"hostId":r.host,"peers":others,
                "catalog":r.catalog,"state":r.state,"sug":r.sug,"chat":r.chat}));
            broadcast(r, Some(id), json!({"t":"joined","id":id,"host":host}));
        }
        "sig" => {
            if let Some(to) = v["to"].as_u64().and_then(|t| r.peers.get(&t)) {
                send(to, json!({"t":"sig","from":id,"data":v["data"]}));
            }
        }
        "stopshare" => broadcast(r, Some(id), json!({"t":"stopshare","id":id})),
        "m3u" if is_host => {
            r.catalog = m3u::parse(v["text"].as_str().unwrap_or(""));
            broadcast(r, None, json!({"t":"catalog","catalog":r.catalog}));
        }
        "state" if is_host => {
            r.state = v.clone();
            broadcast(r, Some(id), v);
        }
        "chat" => {
            let mut message = v["message"].clone();
            if let Some(text) = message["text"].as_str() {
                let clipped: String = text.chars().take(500).collect();
                message["text"] = Value::String(clipped);
                r.chat.push(message.clone());
                if r.chat.len() > 100 { r.chat.remove(0); }
                broadcast(r, None, json!({"t":"chat","message":message}));
            }
        }
        "suggest" => {
            let cut = |k: &str| v[k].as_str().unwrap_or("").chars().take(300).collect::<String>();
            r.sug.push(json!({"title":cut("title"),"url":cut("url"),"by":cut("by")}));
            if r.sug.len() > 50 { r.sug.remove(0); }
            broadcast(r, None, json!({"t":"sug","sug":r.sug}));
        }
        "clearsug" if is_host => {
            r.sug.clear();
            broadcast(r, None, json!({"t":"sug","sug":r.sug}));
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], include_str!("../static/index.html")) }))
        .route("/manifest.webmanifest", get(|| async { ([(header::CONTENT_TYPE, "application/manifest+json")], include_str!("../static/manifest.webmanifest")) }))
        .route("/sw.js", get(|| async { ([(header::CONTENT_TYPE, "text/javascript")], include_str!("../static/sw.js")) }))
        .route("/icon.svg", get(|| async { ([(header::CONTENT_TYPE, "image/svg+xml")], include_str!("../static/icon.svg")) }))
        .route("/ws/{room}", get(ws))
        .with_state(App::default());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let l = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    let address = format!("http://127.0.0.1:{port}");
    println!("FriendCine em {address}");
    if std::env::var_os("FC_NO_BROWSER").is_none() {
        #[cfg(target_os = "windows")]
        let _ = std::process::Command::new("cmd").args(["/C", "start", "", &address]).spawn();
        #[cfg(target_os = "macos")]
        let _ = std::process::Command::new("open").arg(&address).spawn();
        #[cfg(all(unix, not(target_os = "macos")))]
        let _ = std::process::Command::new("xdg-open").arg(&address).spawn();
    }
    axum::serve(l, app).await.unwrap();
}
