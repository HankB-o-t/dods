mod read;
mod write;
#[macro_use] extern crate rocket;
use rocket::{post, serde::json::Json};

#[post("/api/add", format = "json", data = "<entrada>")]
async fn api_add(entrada: Json<write::NewTrad>) -> String {
    write::add_trad("data/en2es.json".to_string(), entrada);

    format!("PALABRA AGREGADAA")
}


#[get("/api/<palabra>")]
async fn api_word(palabra: String) -> String {
        let p = read::readFile(palabra, "data/en2es.json".to_string());
        return p;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![api_word, api_add])
}
