use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tokio_postgres::{NoTls, Error};
use dotenv::dotenv;
use std::env;
use std::sync::{Arc, Mutex};

mod users;
mod global;

async fn establish_db_connection() -> Result<tokio_postgres::Client, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    println!("CONNECTED TO DB");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });
    Ok(client)
}

#[get("/")]
async fn docs() -> impl Responder {
    HttpResponse::Ok().body("Docs will be here.")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client = Arc::new(Mutex::new(
        establish_db_connection().await.expect("Failed to connect to the database"),
    ));

    let counter = web::Data::new(global::AppStateWithCounter {
        client: client.clone(),
    });


    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone()) // <- register the created data
            // .route("/", web::get().to(global::index))
            .service(users::test)
            // .service(web::resource("/test")
            // .route("/test", web::get().to(users::test))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}