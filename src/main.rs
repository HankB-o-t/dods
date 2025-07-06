mod read;
#[macro_use] extern crate rocket;
use std::io::{ stdin, stdout, Write };

#[get("/api/<palabra>")]
async fn api_word(palabra: String) -> String {
        let p = read::readFile(palabra, "data/en2es.json".to_string());
        return p;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![api_word])
}
