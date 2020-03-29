use crate::app_interface::PluginRegistry;
use crate::config::Config;
use colored::*;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn get_host(config: &Config) -> Option<String> { config.server.as_ref()?.get("mount-address").cloned() }
fn get_path(config: &Config) -> Option<String> { config.server.as_ref()?.get("mount-path").cloned() }

#[actix_rt::main]
pub async fn start_server(config: Config, _plugins: PluginRegistry) -> std::result::Result<(), std::io::Error> {
    let host = get_host(&config).unwrap_or("0.0.0.0:1337".to_string());
    let path = get_path(&config).unwrap_or("/".to_string());
    println!("starting {} server on http://{}{}", "APIPlant".bright_blue().bold(), host, path);
    HttpServer::new(move || {
        App::new().service(
            web::scope(&path)
                .route("/models", web::get().to(index))
                .route("/models", web::post().to(index))
                .route("/models/{model_id}", web::get().to(index))
                .route("/models/{model_id}", web::put().to(index))
                .route("/models/{model_id}", web::delete().to(index))

                .route("/models/{model_id}/hooks", web::get().to(index))
                .route("/models/{model_id}/hooks/{hook_id}", web::get().to(index))
                .route("/models/{model_id}/hooks/{hook_id}", web::put().to(index))
                .route("/models/{model_id}/hooks/{hook_id}", web::delete().to(index))

                .route("/models/{model_id}/instances", web::get().to(index))
                .route("/models/{model_id}/instances", web::post().to(index))
                .route("/models/{model_id}/instances/{instance_id}", web::get().to(index))
                .route("/models/{model_id}/instances/{instance_id}", web::put().to(index))
                .route("/models/{model_id}/instances/{instance_id}", web::delete().to(index))
        )
    })
    .bind(host)?
    .run()
    .await
}