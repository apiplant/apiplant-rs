use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn list_definitions() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn create_definitions() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn get_definition() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn update_definition() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn delete_definition() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


pub async fn list_hooks() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn get_hook() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn update_hook() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn delete_hook() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


pub async fn list_instances() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn create_instance() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn get_instance() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn update_instance() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn delete_instance() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}