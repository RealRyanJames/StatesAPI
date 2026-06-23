use mongodb::{bson::doc, options::ClientOptions};
use std::error::Error;

use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn get_home() -> impl Responder {
    HttpResponse::Ok().body("Hello World".to_uppercase())
}

async fn get_client(data: String) -> Result<(), Box<dyn Error>> {
    let option = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = mongodb::Client::with_options(option)?;

    let db = client.database("data");
    let coll = db.collection("states_info");
    coll.insert_one(doc! {"data": data}).await?;

    Ok(())
}

struct State {
    cap: String,
    state: String,
    pop: f32,
}

#[get("/Population/Maine")]
async fn get_maine_pop() -> impl Responder {
    let maine_pop_state = State {
        cap: "Augusta".to_string(),
        state: String::from("Maine"),
        pop: 1.40000 as f32,
    };

    HttpResponse::Ok().body(format!(
        "Population: {} | Capital: {} | State: {} ",
        maine_pop_state.pop, maine_pop_state.cap, maine_pop_state.state
    ));

    get_client(format!(
        "Population: {} | Capital: {} | State: {} ",
        maine_pop_state.pop, maine_pop_state.cap, maine_pop_state.state
    ))
    .await
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
    ));

    get_client(format!(
        "Population: {} | Capital: {} | State: {} ",
        ma_state.pop, ma_state.cap, ma_state.state
    ))
    .await
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
    ));

    get_client(format!(
        "Population: {} | Capital: {} | State: {} ",
        cali_state.pop, cali_state.cap, cali_state.state
    ))
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_home)
            .service(get_pop)
            .service(get_pop_cali)
            .service(get_maine_pop)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
