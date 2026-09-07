use std::cmp;
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

fn lcs(
    archivo_1: &[String],
    archivo_2: &[String],
) -> Vec<Vec<usize>> {
    let m = archivo_1.len();
    let n = archivo_2.len();

    let mut grilla: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];

    for i in 0..m {
        for j in 0..n {
            if archivo_1[i] == archivo_2[j] {
                grilla[i+1][j+1] = grilla[i][j] + 1;
            } else {
                grilla[i+1][j+1] = cmp::max(grilla[i+1][j], grilla[i][j+1]);
            }
        }
    }

    grilla
}

fn main() {
    let ruta_1 = Path::new("archivos/archivo1.txt");
    let ruta_2 = Path::new("archivos/archivo2.txt");

    let archivo_1 = read_file_lines(ruta_1);
    let archivo_2 = read_file_lines(ruta_2);

    let grilla = lcs(&archivo_1, &archivo_2);

    println!("{:?}", archivo_1);
    println!("{:?}", archivo_2);
    println!("{:?}", grilla);
}
