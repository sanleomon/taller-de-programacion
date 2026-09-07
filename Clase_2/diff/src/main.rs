use std::cmp;
use std::fs;
use std::path::Path;

/// Lee un archivo de texto y devuelve sus lineas en un vector de Strings.
fn read_file_lines(ruta: &Path) -> Vec<String> {
    let contenido = fs::read_to_string(ruta).expect("No se pudo leer el archivo");

    contenido.lines().map(|linea| linea.to_string()).collect()
}

/// Construye la grilla de Longest Common Subsequence para dos secuencias de lineas.
fn lcs(archivo_1: &[String], archivo_2: &[String]) -> Vec<Vec<usize>> {
    let m = archivo_1.len();
    let n = archivo_2.len();

    let mut grilla: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];

    for i in 0..m {
        for j in 0..n {
            if archivo_1[i] == archivo_2[j] {
                grilla[i + 1][j + 1] = grilla[i][j] + 1;
            } else {
                grilla[i + 1][j + 1] = cmp::max(grilla[i + 1][j], grilla[i][j + 1]);
            }
        }
    }

    grilla
}

/// Recorre la grilla LCS y muestra las lineas comunes, eliminadas y agregadas.
fn mostrar_diff(grilla: &[Vec<usize>], x: &[String], y: &[String], i: usize, j: usize) {
    if i > 0 && j > 0 && x[i - 1] == y[j - 1] {
        mostrar_diff(grilla, x, y, i - 1, j - 1);
        println!("{}", x[i - 1]);
    } else if j > 0 && (i == 0 || grilla[i][j - 1] >= grilla[i - 1][j]) {
        mostrar_diff(grilla, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if i > 0 && (j == 0 || grilla[i][j - 1] < grilla[i - 1][j]) {
        mostrar_diff(grilla, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    }
}

fn main() {
    let ruta_1 = Path::new("archivos/archivo1.txt");
    let ruta_2 = Path::new("archivos/archivo2.txt");

    let archivo_1 = read_file_lines(ruta_1);
    let archivo_2 = read_file_lines(ruta_2);

    let grilla = lcs(&archivo_1, &archivo_2);

    mostrar_diff(
        &grilla,
        &archivo_1,
        &archivo_2,
        archivo_1.len(),
        archivo_2.len(),
    );
}
