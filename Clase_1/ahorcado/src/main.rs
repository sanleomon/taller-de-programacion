use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum ErrorJuego {
    IntentosAgotados,
}

fn mostrar_palabra(palabra: String, letras_adivinadas: Vec<char>) -> String {
    let mut palabra_mostrada = String::new();
    for letra in palabra.chars() {
        if letras_adivinadas.contains(&letra) {
            palabra_mostrada.push(letra);
        } else {
            palabra_mostrada.push('_');
        }
    }
    palabra_mostrada
}

fn mostrar_palabra_actual(palabra: String, letras_adivinadas: Vec<char>) {
    println!(
        "La palabra hasta el momento es: {}",
        mostrar_palabra(palabra, letras_adivinadas)
    );
}

fn leer_letra() -> Option<char> {
    let mut letra = String::new();
    println!("Ingresa una letra: ");
    io::stdin()
        .read_line(&mut letra)
        .expect("Error leyendo la linea.");
    let letra_limpia = letra.trim();

    if letra_limpia.chars().count() == 1 {
        letra_limpia.chars().next()
    } else {
        None
    }
}

fn procesar_acierto(letra: char, letras_adivinadas: &mut Vec<char>) {
    if !letras_adivinadas.contains(&letra) {
        letras_adivinadas.push(letra);
    } else {
        println!("Ya intentaste con esa letra");
    }
}

fn procesar_fallo(letra: char, letras_falladas: &mut Vec<char>, intentos: &mut i32) {
    if !letras_falladas.contains(&letra) {
        letras_falladas.push(letra);
        *intentos -= 1;
        println!("Fallaste!");
        println!("Fallaste con estas letras: {:?}", letras_falladas);
    } else {
        println!();
        println!("Ya intentaste con esa letra");
        println!("Letras falladas: {:?}", letras_falladas);
    }
}

fn procesar_letra(
    letra: char,
    palabra: String,
    letras_adivinadas: &mut Vec<char>,
    letras_falladas: &mut Vec<char>,
    intentos: &mut i32,
) {
    if palabra.contains(letra) {
        procesar_acierto(letra, letras_adivinadas);
        mostrar_palabra_actual(palabra.clone(), letras_adivinadas.clone());

        println!("Adivinaste las siguientes letras: {:?}", letras_adivinadas);
    } else {
        mostrar_palabra_actual(palabra.clone(), letras_adivinadas.clone());
        procesar_fallo(letra, letras_falladas, intentos);
    }
}

fn mostrar_inicio_partida(palabra: String, letras_adivinadas: Vec<char>, intentos: i32) {
    println!();
    mostrar_palabra_actual(palabra, letras_adivinadas);
    println!("Adivinaste las siguientes letras: ");
    println!("Te quedan {} intentos", intentos);
}

fn jugar(palabra: String) -> Result<(), ErrorJuego> {
    let mut intentos = 5;
    let mut letras_adivinadas = Vec::<char>::new();
    let mut letras_falladas = Vec::<char>::new();

    mostrar_inicio_partida(palabra.clone(), letras_adivinadas.clone(), intentos);

    while intentos > 0 && mostrar_palabra(palabra.clone(), letras_adivinadas.clone()) != palabra {
        if let Some(letra) = leer_letra() {
            procesar_letra(
                letra,
                palabra.clone(),
                &mut letras_adivinadas,
                &mut letras_falladas,
                &mut intentos,
            );
        } else {
            println!("Debes ingresar exactamente una letra.");
        }

        if intentos > 0 {
            println!("Te quedan {} intentos", intentos);
        }
    }

    if intentos > 0 {
        Ok(())
    } else {
        Err(ErrorJuego::IntentosAgotados)
    }
}

fn leer_palabras() -> Vec<String> {
    let archivo = File::open("palabras.txt").expect("No se pudo abrir el archivo");
    let lector = BufReader::new(archivo);
    let mut palabras_leidas = Vec::<String>::new();

    for linea in lector.lines() {
        match linea {
            Ok(palabra_leida) => {
                palabras_leidas.push(palabra_leida);
            }
            Err(error) => {
                println!("Error leyendo una línea: {:?}", error);
            }
        }
    }
    palabras_leidas
}

fn main() {
    let palabras_leidas = leer_palabras();

    println!("Bienvenido al ahorcado de FIUBA!");

    for palabra in palabras_leidas {
        match jugar(palabra) {
            Ok(()) => {
                println!("Acertaste la palabra!");
            }
            Err(ErrorJuego::IntentosAgotados) => {
                println!("Te quedaste sin intentos.");
            }
        }
    }
}
