/*
This file is for the hello_world handler.
 */

pub async fn hello_world() -> String {
    "hello, bongus!!!".to_owned()
    // why use to_owned() when returning a string?
    // because we're assuming the user of this function will 
    // want to modify the value we're returning 
    // if we assume they won't modify the value 
    // we can return an &str reference
}