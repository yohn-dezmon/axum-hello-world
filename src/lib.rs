mod routes;
use routes::create_routes;
use axum::{
    Router,
};

 pub async fn run() {
    // build our server/application
    // only a single route for now 
    // I think the || is a lambda function (anonymous func)
    // he called || a "closure"
    let app: Router = create_routes();

    // run it with hyper on localhost:3000
    // 0.0.0.0 makes it compatible with docker containers
    // unwrap is useful incase it fails
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
 }
