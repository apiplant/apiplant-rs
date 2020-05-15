mod controller;
use controller::*;
use crate::app_interface::PluginRegistry;
use crate::config::Config;
use colored::*;
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
pub async fn start_server(config: Config, _plugins: PluginRegistry) -> std::result::Result<(), std::io::Error> {
    let address = config.server.mount_address;
    let path = config.server.mount_path;
    println!("starting {} server on http://{}{}", "APIPlant".bright_blue().bold(), address, path);
    HttpServer::new(move || {
        App::new()
            .service(web::scope(&path)
                .route("", web::get().to(index))
                .route("/models", web::get().to(list_definitions))
                .route("/models", web::post().to(create_definitions))
                .route("/models/{model_id}", web::get().to(get_definition))
                .route("/models/{model_id}", web::put().to(update_definition))
                .route("/models/{model_id}", web::delete().to(delete_definition))

                .route("/models/{model_id}/hooks", web::get().to(list_hooks))
                .route("/models/{model_id}/hooks/{hook_id}", web::get().to(get_hook))
                .route("/models/{model_id}/hooks/{hook_id}", web::put().to(update_hook))
                .route("/models/{model_id}/hooks/{hook_id}", web::delete().to(delete_hook))

                .route("/models/{model_id}/instances", web::get().to(list_instances))
                .route("/models/{model_id}/instances", web::post().to(create_instance))
                .route("/models/{model_id}/instances/{instance_id}", web::get().to(get_instance))
                .route("/models/{model_id}/instances/{instance_id}", web::put().to(update_instance))
                .route("/models/{model_id}/instances/{instance_id}", web::delete().to(delete_instance))
            )
    })
    .bind(address)?
    .run()
    .await
}