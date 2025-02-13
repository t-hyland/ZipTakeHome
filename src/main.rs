// Thomas Hyland Backend Take Home Assignment Submission
// Edits in this file include the 2 .route additions to route to the 
//  proper functions in the additions.rs file where I wrote the functions for 
//  the POST and GET routing. 

use axum::{
    routing::{get,post}, 
    Router,
};

pub mod routes;

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(routes::hello::hello_world))
        .route("/api/jamf/credentials", post(routes::additions::credentials))
        .route("/api/jamf/devices", get(routes::additions::devices));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}