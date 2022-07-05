use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

struct Page {
    id: u32,
    name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

#[get("/{name}")]
async fn home(name: web::Path<String>) -> impl Responder {
    format!("Welcome asshole {name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        app_name: String::from("fet v0.1.0"),
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(home)
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
