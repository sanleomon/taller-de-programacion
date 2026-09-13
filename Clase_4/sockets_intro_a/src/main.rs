use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Ejecuta la logica del cliente y responde al saludo del servidor.
fn ejecutar_cliente() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("No se pudo conectar al servidor");

    println!("Cliente conectado");

    let mut buffer = [0; 1024];
    let bytes_leidos = stream.read(&mut buffer).expect("Error al leer el saludo");

    let mensaje = String::from_utf8_lossy(&buffer[..bytes_leidos]);
    println!("Cliente recibió: {}", mensaje);

    stream
        .write_all("Buen día Papá".as_bytes())
        .expect("Error al enviar la respuesta");
}

/// Acepta un cliente, envia el saludo y recibe su respuesta.
fn ejecutar_servidor(listener: TcpListener) {
    let (mut stream, direccion) = listener.accept().expect("Error al aceptar la conexión");

    println!("Cliente conectado desde {}", direccion);

    stream
        .write_all(b"Hola hijo")
        .expect("Error al enviar el saludo");

    let mut buffer = [0; 1024];
    let bytes_leidos = stream
        .read(&mut buffer)
        .expect("Error al leer la respuesta");

    let mensaje = String::from_utf8_lossy(&buffer[..bytes_leidos]);
    println!("Servidor recibió: {}", mensaje);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("No se pudo iniciar el servidor");

    println!("Servidor escuchando en 127.0.0.1:8080");

    let cliente = thread::spawn(ejecutar_cliente);

    ejecutar_servidor(listener);

    cliente.join().unwrap();
}
