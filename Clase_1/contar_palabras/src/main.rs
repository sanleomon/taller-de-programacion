use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn contar_frecuencias() -> HashMap<String, i32> {
    let mut frecuencias: HashMap<String, i32> = HashMap::new();

    let archivo = File::open("texto.txt").expect("No se pudo abrir el archivo");
    let lector = BufReader::new(archivo);

    for linea in lector.lines() {
        let linea = linea.expect("Error al leer la línea");
        let linea = linea.to_lowercase();

        for palabra in linea.split_whitespace() {
            let palabra = palabra
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string();

            if palabra.is_empty() {
                continue;
            }

            let contador = frecuencias.entry(palabra).or_insert(0);
            *contador += 1;
        }
    }
    frecuencias
}

fn ordenar_frecuencias(frecuencias: &HashMap<String, i32>) -> Vec<(&String, &i32)> {
    let mut palabras: Vec<(&String, &i32)> = frecuencias.iter().collect();
    palabras.sort_by(|a, b| b.1.cmp(a.1));
    palabras
}

fn main() {
    let frecuencias = contar_frecuencias();
    let palabras = ordenar_frecuencias(&frecuencias);

    println!();

    for (palabra, frecuencia) in palabras {
        println!("{} -> {}", palabra, frecuencia);
    }

    println!();
}
