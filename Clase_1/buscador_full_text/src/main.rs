use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Obtiene el identificador numerico de un documento a partir
/// del nombre del archivo.
fn obtener_id_documento(ruta: &Path) -> i32 {
    ruta.file_stem()
        .expect("El archivo no tiene nombre")
        .to_string_lossy()
        .parse::<i32>()
        .expect("El nombre del archivo no es un id válido")
}

/// Lee el contenido de un documento y lo convierte a minusculas.
fn leer_documento(ruta: &Path) -> String {
    let contenido = fs::read_to_string(ruta).expect("No se pudo leer el archivo");
    contenido.to_lowercase()
}

/// Agrega una palabra al indice invertido o incrementa su frecuencia
/// si ya aparece en el documento indicado.
fn agregar_al_indice(
    indice: &mut HashMap<String, Vec<(i32, i32)>>,
    palabra: String,
    id_documento: i32,
) {
    let documentos = indice.entry(palabra).or_default();

    if let Some((_, frecuencia)) = documentos.iter_mut().find(|(id, _)| *id == id_documento) {
        *frecuencia += 1;
    } else {
        documentos.push((id_documento, 1));
    }
}

/// Procesa las palabras de un documento, elimina las stop words
/// y agrega los terminos validos al indice invertido.
fn procesar_documento(
    contenido: &str,
    id_documento: i32,
    indice: &mut HashMap<String, Vec<(i32, i32)>>,
    stop_words: &[&str],
) {
    for palabra in contenido.split_whitespace() {
        let palabra = palabra
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();

        if palabra.is_empty() {
            continue;
        }

        if stop_words.contains(&palabra.as_str()) {
            continue;
        }

        agregar_al_indice(indice, palabra, id_documento);
    }
}

/// Recorre los documentos del corpus, los procesa y construye
/// el indice invertido. Devuelve la cantidad de documentos procesados.
fn indexar_corpus(indice: &mut HashMap<String, Vec<(i32, i32)>>, stop_words: &[&str]) -> usize {
    let archivos = fs::read_dir("corpus").expect("No se pudo leer el directorio corpus");

    let mut cantidad_documentos = 0;

    for archivo in archivos {
        let archivo = archivo.expect("No se pudo leer una entrada del directorio");
        let ruta = archivo.path();

        let id_documento = obtener_id_documento(&ruta);
        let contenido = leer_documento(&ruta);

        procesar_documento(&contenido, id_documento, indice, stop_words);
        cantidad_documentos += 1;
    }

    cantidad_documentos
}

/// Solicita al usuario una frase de busqueda y devuelve
/// la entrada sin espacios al principio ni al final.
fn leer_consulta() -> String {
    print!("Introduce una frase: ");
    io::stdout().flush().unwrap();

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Error al leer la línea");

    let frase_limpia: String = entrada.trim().to_string();
    frase_limpia
}

/// Procesa la consulta, elimina las stop words y devuelve
/// los terminos validos para realizar la busqueda.
fn procesar_consulta(consulta: &str, stop_words: &[&str]) -> Vec<String> {
    let consulta = consulta.to_lowercase();
    let mut terminos: Vec<String> = Vec::new();

    for palabra in consulta.split_whitespace() {
        let palabra = palabra
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();

        if palabra.is_empty() {
            continue;
        }

        if stop_words.contains(&palabra.as_str()) {
            continue;
        }

        terminos.push(palabra);
    }

    terminos
}

/// Busca los documentos que contienen los terminos de la consulta
/// y calcula su puntaje sumando las frecuencias de los terminos.
fn buscar_documentos(
    terminos: &[String],
    indice: &HashMap<String, Vec<(i32, i32)>>,
) -> HashMap<i32, i32> {
    let mut resultados: HashMap<i32, i32> = HashMap::new();

    for termino in terminos {
        if let Some(documentos) = indice.get(termino) {
            for (id_documento, frecuencia) in documentos {
                let puntaje = resultados.entry(*id_documento).or_insert(0);
                *puntaje += *frecuencia;
            }
        }
    }
    resultados
}

/// Busca los documentos y calcula su puntaje de relevancia
/// utilizando la frecuencia de los terminos y el calculo tf-idf.
fn buscar_documentos_tfidf(
    terminos: &[String],
    indice: &HashMap<String, Vec<(i32, i32)>>,
    cantidad_documentos: usize,
) -> HashMap<i32, f64> {
    let mut resultados: HashMap<i32, f64> = HashMap::new();

    for termino in terminos {
        if let Some(documentos) = indice.get(termino) {
            let frecuencia_documental = documentos.len();
            let idf = (cantidad_documentos as f64 / (1 + frecuencia_documental) as f64).ln();

            for (id_documento, frecuencia) in documentos {
                let puntaje_termino = *frecuencia as f64 * idf;

                let puntaje = resultados.entry(*id_documento).or_insert(0.0);
                *puntaje += puntaje_termino;
            }
        }
    }

    resultados
}

/// Ordena los resultados por puntaje de mayor a menor
/// y los muestra por pantalla.
fn mostrar_resultados(resultados: &HashMap<i32, i32>) {
    let mut resultados_ordenados: Vec<(&i32, &i32)> = resultados.iter().collect();

    resultados_ordenados.sort_by(|a, b| b.1.cmp(a.1));

    for (id_documento, puntaje) in resultados_ordenados {
        println!("Documento {} - Puntaje: {}", id_documento, puntaje);
    }
}

/// Ordena los resultados tf-idf de mayor a menor
/// y los muestra por pantalla.
fn mostrar_resultados_tfidf(resultados: &HashMap<i32, f64>) {
    let mut resultados_ordenados: Vec<(&i32, &f64)> = resultados.iter().collect();

    resultados_ordenados.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (id_documento, puntaje) in resultados_ordenados {
        println!(
            "Documento {} - Puntaje tf-idf: {:.4}",
            id_documento, puntaje
        );
    }
}

fn main() {
    let mut indice: HashMap<String, Vec<(i32, i32)>> = HashMap::new();

    let stop_words = [
        "el", "la", "los", "las", "un", "una", "uno", "a", "ante", "bajo", "con", "de", "del",
        "desde", "en", "para", "por", "sin", "y", "e", "ni", "o", "u", "pero", "porque", "yo",
        "tú", "él", "ella", "esto", "eso", "que", "es", "son", "se", "su", "sus",
    ];

    let cantidad_documentos = indexar_corpus(&mut indice, &stop_words);

    let consulta = leer_consulta();
    let terminos = procesar_consulta(&consulta, &stop_words);

    let resultados = buscar_documentos(&terminos, &indice);

    mostrar_resultados(&resultados);

    let resultados_tfidf = buscar_documentos_tfidf(&terminos, &indice, cantidad_documentos);

    mostrar_resultados_tfidf(&resultados_tfidf);
}
