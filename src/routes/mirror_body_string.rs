/*
This handler will return whatever the user passes into the 
Body of the POST request.
(don't forget to update GET --> POST in postman.)
 */
pub async fn mirror_body_string(body: String) -> String {
    body
}