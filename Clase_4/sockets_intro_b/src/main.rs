use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Atiende una conexion del servidor, enviando un saludo y leyendo la respuesta.
fn atender_cliente(mut stream: TcpStream) {
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

/// Ejecuta la logica de un cliente que se conecta al servidor.
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

/// Crea la cantidad indicada de clientes concurrentes.
fn crear_clientes(cantidad: usize) -> Vec<thread::JoinHandle<()>> {
    let mut clientes = Vec::new();

    for _ in 0..cantidad {
        clientes.push(thread::spawn(ejecutar_cliente));
    }

    clientes
}

/// Acepta y atiende concurrentemente la cantidad indicada de clientes.
fn aceptar_clientes(listener: &TcpListener, cantidad: usize) -> Vec<thread::JoinHandle<()>> {
    let mut servidores = Vec::new();

    for _ in 0..cantidad {
        let (stream, direccion) = listener.accept().expect("Error al aceptar la conexión");

        println!("Cliente conectado desde {}", direccion);

        servidores.push(thread::spawn(move || {
            atender_cliente(stream);
        }));
    }

    servidores
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("No se pudo iniciar el servidor");

    println!("Servidor escuchando en 127.0.0.1:8080");

    let clientes = crear_clientes(2);
    let servidores = aceptar_clientes(&listener, 2);

    for cliente in clientes {
        cliente.join().unwrap();
    }

    for servidor in servidores {
        servidor.join().unwrap();
    }
}
