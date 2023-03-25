/*
This handler will return whatever the user passes into the 
Body of the POST request.
(don't forget to update GET --> POST in postman.)
 */
use axum::Json;
use serde::{Serialize, Deserialize};

// I believe derive here is from `serde`
// not exactly sure what this line does currently
#[derive(Serialize, Deserialize)]
pub struct MirrorJson {
    message: String,
}

// Serialize allows you to convert from a Rust Struct to a String
#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    // this just returns the input JSON
    // Json(body)
    Json(MirrorJsonResponse { message: body.message,
                              message_from_server: "Hello from server".to_owned()})
}