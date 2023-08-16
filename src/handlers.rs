use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use zen_engine::DecisionEngine;
use zen_engine::loader::{FilesystemLoader, FilesystemLoaderOptions};

#[derive(Debug, serde::Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Request {
    id: u32,
    name: String,
    data: Value
}

const RESOURCES_PATH: &str = "./resources";

pub async fn evaluate(body: web::Json<Request>) -> impl Responder {
    println!("id: {:?}, name: {:?}, body: {:?}", body.id, body.name, body.data);
    let engine = DecisionEngine::new(
        FilesystemLoader::new(
            FilesystemLoaderOptions {
                keep_in_memory: true,
                root: "./"
            }
        )
    );

    let result = engine
        .evaluate(RESOURCES_PATH.to_owned() + "/decisions/test.json", &body.data)
        .await
        .unwrap();
    println!("result: {:?}", result);
    HttpResponse::Ok().json(result)
}

