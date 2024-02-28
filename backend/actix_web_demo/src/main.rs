use actix_web::{get,post,App,HttpServer,Responder,web,HttpResponse};
use serde::{Deserialize, Serialize}; // Add this line


#[derive(Debug, Deserialize, Serialize)] // Update this line
struct User {
    username: String,
    email: String,
}

#[post("/create_user")]
async fn create_user(user : web::Json<User>) -> impl Responder {
    let new_user = user.into_inner();
    
    HttpResponse::Created().json(new_user)

}


#[get("/")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[get("/user/{id}/{name}")]
async fn user_info(info: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = info.into_inner();
    HttpResponse::Ok().body(format!("User ID: {}, Name: {}", id, name))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(user_info)
        .service(create_user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
