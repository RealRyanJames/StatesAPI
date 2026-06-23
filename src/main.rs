use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("Hello World".to_uppercase())
}

struct State {
    cap: String,
    state: String,
    pop: f32,
}

#[get("/Population/MA")]
async fn get_pop() -> impl Responder {
    let ma_state = State {
        cap: "Boston".to_string(),
        state: String::from("Massachusetts"),
        pop: 7.154,
    };

    HttpResponse::Ok().body(format!(
        "Population: {} | Capital: {} | State: {} ",
        ma_state.pop, ma_state.cap, ma_state.state
    ))
}

#[get("/Population/Califonia")]
async fn get_pop_cali() -> impl Responder {
    let cali_state = State {
        cap: "Sacromentro".to_string(),
        state: String::from("California"),
        pop: 39.431263 as f32,
    };

    HttpResponse::Ok().body(format!(
        "Population: {} | Capital: {} | State: {} ",
        cali_state.pop, cali_state.cap, cali_state.state
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_home)
            .service(get_pop)
            .service(get_pop_cali)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
