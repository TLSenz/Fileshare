use axum::{
    routing::{get, },
     Router,
};
use axum::routing::post;
use crate::controller::usercontroller::{signup, upload_file};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world) )
        .route("/api/signup", post(signup))
        .route("/api/upload", post(upload_file));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str{
    "Hello World".as_ref()
}


pub mod controller{
    pub mod usercontroller;
}
pub mod model{
    pub mod usermodel;
}
pub mod repository{
    pub mod userrepository;
}
pub mod service{
    pub mod userservice;
}
pub mod schema;



