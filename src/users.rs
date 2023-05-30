use actix_web::{get, web, HttpResponse, Responder};

struct Planet {
    id: i32,
    name: String,
    size: String,
}
#[get("/test")]
pub async fn test(_req_body: String, data: web::Data<crate::global::AppStateWithCounter>) -> impl Responder {
    let client = data.client.lock().unwrap();
    // Use the client for testing purposes
    // Example: client.query("SELECT * FROM users", &[]).await
    let result = client.query("SELECT * FROM planets;", &[]).await;
    // Process the query results and generate a response
    HttpResponse::Ok()

}

impl From<&tokio_postgres::Row> for Planet {
    fn from(row: &tokio_postgres::Row) -> Self {
        Planet {
            id: row.get("planet_id"),
            name: row.get("planet_name"),
            size: row.get("planet_size"),
        }
    }
}
