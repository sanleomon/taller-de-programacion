use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::{sync::mpsc, thread};

/// Cuenta las frecuencias de palabras de un archivo.
fn contar_frecuencias(ruta: &str) -> HashMap<String, i32> {
    let mut frecuencias: HashMap<String, i32> = HashMap::new();

    let archivo = File::open(ruta).expect("No se pudo abrir el archivo");
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

/// Ordena las palabras de mayor a menor frecuencia.
fn ordenar_frecuencias(frecuencias: &HashMap<String, i32>) -> Vec<(&String, &i32)> {
    let mut palabras: Vec<(&String, &i32)> = frecuencias.iter().collect();
    palabras.sort_by(|a, b| b.1.cmp(a.1));
    palabras
}

/// Procesa un archivo y actualiza directamente el mapa global compartido.
fn procesar_archivo_global(ruta: &str, frecuencias: &Arc<Mutex<HashMap<String, i32>>>) {
    let archivo = File::open(ruta).expect("No se pudo abrir el archivo");
    let lector = BufReader::new(archivo);

    for linea in lector.lines() {
        let linea = linea.expect("Error al leer la línea").to_lowercase();

        for palabra in linea.split_whitespace() {
            let palabra = palabra
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_string();

            if palabra.is_empty() {
                continue;
            }

            let mut mapa = frecuencias.lock().unwrap();
            let contador = mapa.entry(palabra).or_insert(0);
            *contador += 1;
        }
    }
}

/// Procesa los archivos concurrentemente y combina los resultados mediante join.
fn ejecutar_con_join(rutas: &[&str]) -> HashMap<String, i32> {
    let mut handles = Vec::new();

    for ruta in rutas {
        let ruta = (*ruta).to_string();

        let handle = thread::spawn(move || contar_frecuencias(&ruta));

        handles.push(handle);
    }

    let mut frecuencias_totales = HashMap::new();

    for handle in handles {
        let frecuencias_parciales = handle.join().unwrap();

        for (palabra, frecuencia) in frecuencias_parciales {
            let contador = frecuencias_totales.entry(palabra).or_insert(0);
            *contador += frecuencia;
        }
    }

    frecuencias_totales
}

/// Procesa los archivos concurrentemente y combina los resultados mediante channels.
fn ejecutar_con_channels(rutas: &[&str]) -> HashMap<String, i32> {
    let (tx, rx) = mpsc::channel();

    for ruta in rutas {
        let ruta = (*ruta).to_string();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let frecuencias = contar_frecuencias(&ruta);
            tx_clone.send(frecuencias).unwrap();
        });
    }

    drop(tx);

    let mut frecuencias_totales: HashMap<String, i32> = HashMap::new();

    for frecuencias_parciales in rx {
        for (palabra, frecuencia) in frecuencias_parciales {
            let contador = frecuencias_totales.entry(palabra).or_insert(0);
            *contador += frecuencia;
        }
    }

    frecuencias_totales
}

/// Procesa los archivos utilizando un mapa global compartido entre threads.
fn ejecutar_con_mapa_global(rutas: &[&str]) -> HashMap<String, i32> {
    let frecuencias_totales = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for ruta in rutas {
        let ruta = (*ruta).to_string();
        let frecuencias_compartidas = Arc::clone(&frecuencias_totales);

        let handle = thread::spawn(move || {
            procesar_archivo_global(&ruta, &frecuencias_compartidas);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mutex = Arc::try_unwrap(frecuencias_totales).expect("Todavía existen referencias al mapa");

    mutex.into_inner().unwrap()
}

fn main() {
    let rutas = vec![
        "archivos/archivo1.txt",
        "archivos/archivo2.txt",
        "archivos/archivo3.txt",
    ];

    let inicio = Instant::now();
    let resultado_join = ejecutar_con_join(&rutas);
    let duracion_join = inicio.elapsed();

    let inicio = Instant::now();
    let resultado_channels = ejecutar_con_channels(&rutas);
    let duracion_channels = inicio.elapsed();

    let inicio = Instant::now();
    let resultado_mapa_global = ejecutar_con_mapa_global(&rutas);
    let duracion_mapa_global = inicio.elapsed();

    assert_eq!(resultado_join, resultado_channels);
    assert_eq!(resultado_join, resultado_mapa_global);

    println!("Tiempo join: {:?}", duracion_join);
    println!("Tiempo channels: {:?}", duracion_channels);
    println!("Tiempo mapa global: {:?}", duracion_mapa_global);

    let palabras = ordenar_frecuencias(&resultado_join);

    for (palabra, frecuencia) in palabras {
        println!("{} -> {}", palabra, frecuencia);
    }
}
