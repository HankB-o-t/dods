use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::read;

#[derive(Deserialize)]
pub struct NewTrad {
    pub EN: String,
    pub ES: String,
}

pub fn add_trad(ruta: String, entrada: Json<NewTrad>) {
    let mut traducciones: Vec<read::Word> = if Path::new(&ruta).exists() {
        let file = File::open(&ruta)
            .expect("No se pudo abrir el JSON");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        Vec::new()
    };

    let nueva = read::Word {
        ID: traducciones.len() as i32 + 1,
        EN: entrada.EN.clone(),
        ES: entrada.ES.clone(),
    };

    traducciones.push(nueva);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&ruta)
        .expect("No se pudo abrir para escribir");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &traducciones)
        .expect("No se pudo escribir el JSON");
}
