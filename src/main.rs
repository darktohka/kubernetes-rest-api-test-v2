use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Cursor;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
struct Video {
    #[validate(length(min = 1, max = 100))]
    title: String,

    #[validate(length(max = 500))]
    description: String,

    #[validate(length(min = 1))]
    owner_user_id: String,
}

async fn get_videos() -> impl Responder {
    let client_options = ClientOptions::parse(&env::var("MONGO_CONNECTION_STRING").unwrap())
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("videos");
    let collection = db.collection::<Video>("videos");

    let video_cursor: Cursor<Video> = collection.find(None, None).await.unwrap();
    let videos: Vec<Video> = video_cursor.try_collect().await.unwrap();

    HttpResponse::Ok().json(videos)
}

async fn add_video(video: web::Json<Video>) -> impl Responder {
    let validation = video.validate();

    if let Err(error) = validation {
        return HttpResponse::BadRequest().json(json!({"error": error}));
    }

    let client_options: ClientOptions =
        ClientOptions::parse(&env::var("MONGO_CONNECTION_STRING").unwrap())
            .await
            .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("videos");
    let collection = db.collection::<Video>("videos");

    let video_creation = collection
        .insert_one(video.into_inner(), None)
        .await
        .unwrap();
    let video_id = video_creation.inserted_id.as_object_id().unwrap();
    let video = collection
        .find_one(Some(doc! {"_id": video_id}), None)
        .await
        .unwrap();

    HttpResponse::Ok().json(video)
}

async fn get_version() -> impl Responder {
    HttpResponse::Ok().json(json!({"version": 2, "description": "This is the second API, in Rust"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/videos", web::get().to(get_videos))
            .route("/videos", web::post().to(add_video))
            .route("/version", web::get().to(get_version))
    })
    .bind("0.0.0.0:5001")?
    .run()
    .await
}
