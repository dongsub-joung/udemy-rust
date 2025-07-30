use actix_web::{App, HttpServer};

struct User{
    user_name: String,
    email: String,
    age: i32,
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(new_user)
    })
    .bind(("0.0.0.0", 4000). expect("failed")
    .run()
    .await
}
