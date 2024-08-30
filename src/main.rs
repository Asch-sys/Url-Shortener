use actix_web::{web, App, HttpServer, HttpResponse, Responder, post};
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sled::Db;
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct UrlData {
    url: String,
}

#[derive(Serialize, Deserialize)]
struct ShortenedUrl {
    shortened_url: String,
}

#[derive(Clone)]
struct AppState {
    db: Db,
}

#[post("/shorten")]
async fn shorten_url(data: web::Json<UrlData>, state: web::Data<Mutex<AppState>>) -> impl Responder {
    let short_code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    let db = &state.lock().unwrap().db;
    db.insert(short_code.as_bytes(), data.url.as_bytes()).unwrap();

    let shortened_url = format!("https://your-vercel-domain.vercel.app/{}", short_code);

    HttpResponse::Ok().json(ShortenedUrl { shortened_url })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = sled::open("url_shortener_db").unwrap();
    let state = web::Data::new(Mutex::new(AppState { db }));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(shorten_url)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
