use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use chrono::{NaiveDateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    query,
};
use std::io::Error;

#[derive(Serialize, Clone)]
struct Mind {
    id: u32,
    publish_time: NaiveDateTime,
    content: String,
}

struct AppState {
    db_pool: PgPool,
}

#[derive(Serialize, Clone)]
struct GetMindsResponse {
    minds: Vec<Mind>,
}

#[get("/minds")]
async fn get_minds_handler(state: Data<AppState>) -> Result<impl Responder, Error> {
    let result = query("SELECT id, publish_time, content FROM minds;")
        .fetch_all(&state.db_pool)
        .await
        .unwrap();
    let mut minds: Vec<Mind> = vec![];

    for item in result {
        minds.push(Mind {
            id: item.get::<i32, _>(0) as u32,
            publish_time: item.get(1),
            content: item.get(2),
        });
    }

    Ok(Json(GetMindsResponse { minds }))
}

#[derive(Deserialize)]
struct CreateMindHandlerRequest {
    content: String,
}

#[post("/minds")]
async fn create_mind_handler(
    state: Data<AppState>,
    data: Json<CreateMindHandlerRequest>,
) -> Result<impl Responder, Error> {
    let mind = Mind {
        id: 1,
        publish_time: Utc::now().naive_utc().with_nanosecond(0).unwrap(),
        content: data.content.clone(),
    };

    query("INSERT INTO minds (publish_time, content) VALUES ($1, $2);")
        .bind(mind.publish_time)
        .bind(mind.content.clone())
        .execute(&state.db_pool)
        .await
        .unwrap();

    Ok(Json(mind))
}

#[delete("/minds/{id}")]
async fn delete_mind_handler(
    state: Data<AppState>,
    path: Path<u32>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();

    query("DELETE FROM minds WHERE id = $1")
        .bind(id as i32)
        .execute(&state.db_pool)
        .await
        .unwrap();

    Ok(HttpResponse::NoContent())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Data::new(AppState {
        db_pool: PgPoolOptions::new()
            .max_connections(3)
            .connect("postgres://mindbox:mindbox@localhost:5432/mindbox")
            .await
            .unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_minds_handler)
            .service(create_mind_handler)
            .service(delete_mind_handler)
    })
    .workers(1)
    .shutdown_timeout(10)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
