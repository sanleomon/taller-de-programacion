use std::fs;
use std::path::Path;

fn read_file_lines(ruta: &Path) -> Vec<String> {
    let contenido = fs::read_to_string(ruta)
        .expect("No se pudo leer el archivo");

    contenido
        .lines()
        .map(|linea| linea.to_string())
        .collect()
}

fn main() {
    let ruta_1 = Path::new("archivos/archivo1.txt");
    let ruta_2 = Path::new("archivos/archivo2.txt");

    let archivo_1 = read_file_lines(ruta_1);
    let archivo_2 = read_file_lines(ruta_2);

    println!("{:?}", archivo_1);
    println!("{:?}", archivo_2);
}
