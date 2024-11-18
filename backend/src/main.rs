use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use chrono::{DateTime, TimeZone, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::{io::Error, sync::Mutex};

#[derive(Serialize, Clone)]
struct Mind {
    id: u32,
    publish_time: DateTime<Utc>,
    content: String,
}

struct AppState {
    minds: Mutex<Vec<Mind>>,
}

#[derive(Serialize, Clone)]
struct GetMindsResponse {
    minds: Vec<Mind>,
}

#[get("/minds")]
async fn get_minds_handler(state: Data<AppState>) -> Result<impl Responder, Error> {
    Ok(Json(GetMindsResponse {
        minds: state.minds.lock().unwrap().to_vec(),
    }))
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
    let max_id = match state.minds.lock().unwrap().iter().map(|x| x.id).max() {
        Some(x) => x,
        None => 0,
    };
    let mind = Mind {
        id: max_id + 1,
        publish_time: Utc::now().with_nanosecond(0).unwrap(),
        content: data.content.clone(),
    };

    state.minds.lock().unwrap().push(mind.clone());

    Ok(Json(mind))
}

#[delete("/minds/{id}")]
async fn delete_mind_handler(
    state: Data<AppState>,
    path: Path<u32>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();

    state.minds.lock().unwrap().retain(|x| x.id != id);

    Ok(HttpResponse::NoContent())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Data::new(AppState {
        minds: Mutex::new(vec![
            Mind {
                id: 1,
                publish_time: Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(),
                content: "This is a test content.".to_string(),
            },
            Mind {
                id: 2,
                publish_time: Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap(),
                content: "普普通通的第二段测试内容。".to_string(),
            },
            Mind {
                id: 3,
                publish_time: Utc.with_ymd_and_hms(2024, 10, 31, 23, 59, 59).unwrap(),
                content: "写点什么好呢？".to_string(),
            },
        ]),
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
