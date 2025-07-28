use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, Write};

#[derive(Deserialize, Serialize, Debug)]
pub struct Word {
    pub ID: i32, //podria ser u32, pero por las dudas
    pub EN: String,
    pub ES: String,
}

pub fn readFile(palabra: String, dir: String) -> String {
    let file = File::open(dir)
        .expect("El puto json no se abrio");
    let reader = BufReader::new(file);

    let traducciones: Vec<Word> = serde_json::from_reader(reader)
        .expect("error para parsear el puto json");

    let resultado = traducciones
        .iter()
        .find(|t| t.EN.to_lowercase() == palabra);

    match resultado {
        Some(t) => return t.ES.clone(),
        None => return "\n--------No se encontro la palabra--------\nChequea la ortografia, o  quizas,  si  la\npalabra es muy de nicho, no va a aparecer\npodes            agregarla           sino\nUsando el comando:\ncurl -X POST <url> -H 'Content-Type: application/json -d ' \"{\"EN\":\"palabra\", \"ES\":\"traducida\"}\" ".to_string(),
    }
}
