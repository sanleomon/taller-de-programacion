use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;

/// Lee el nickname ingresado por el usuario.
fn leer_nickname() -> String {
    println!("Ingrese nickname:");

    let mut nickname = String::new();

    io::stdin()
        .read_line(&mut nickname)
        .expect("Error al leer el nickname");

    nickname.trim().to_string()
}

/// Lee y valida el puerto ingresado por el usuario.
fn leer_puerto() -> u16 {
    println!("Ingrese puerto:");

    let mut puerto = String::new();

    io::stdin()
        .read_line(&mut puerto)
        .expect("Error al leer el puerto");

    puerto.trim().parse().expect("El puerto debe ser un número")
}

/// Crea el socket UDP utilizado por el chat.
fn crear_socket(puerto: u16) -> UdpSocket {
    let direccion = format!("0.0.0.0:{}", puerto);

    let socket = UdpSocket::bind(&direccion).expect("No se pudo abrir el socket");

    println!("Socket abierto en {}", direccion);

    socket
}

/// Envia el nickname mediante broadcast.
fn anunciar_nickname(socket: &UdpSocket, nickname: &str, puerto: u16) {
    socket
        .set_broadcast(true)
        .expect("No se pudo habilitar broadcast");

    let destino = format!("255.255.255.255:{}", puerto);
    let anuncio = format!("NICK:{}", nickname);

    socket
        .send_to(anuncio.as_bytes(), &destino)
        .expect("Error al enviar broadcast");
}

/// Registra un nickname junto con la direccion de origen.
fn registrar_usuario(
    mensaje: &str,
    origen: SocketAddr,
    usuarios: &Arc<Mutex<HashMap<String, SocketAddr>>>,
) {
    let nickname = mensaje.strip_prefix("NICK:").unwrap().to_string();

    let mut usuarios = usuarios.lock().unwrap();
    usuarios.insert(nickname, origen);

    println!("Anuncio recibido de {}: {}", origen, mensaje);
}

/// Escucha anuncios y mensajes recibidos por UDP.
fn iniciar_receptor(
    socket: UdpSocket,
    usuarios: Arc<Mutex<HashMap<String, SocketAddr>>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer = [0u8; 1024];

        loop {
            let (bytes_leidos, origen) = socket
                .recv_from(&mut buffer)
                .expect("Error al recibir mensaje");

            let mensaje = String::from_utf8_lossy(&buffer[..bytes_leidos]);

            if mensaje.starts_with("NICK:") {
                registrar_usuario(&mensaje, origen, &usuarios);
            } else {
                println!("Mensaje recibido de {}: {}", origen, mensaje);
            }
        }
    })
}

/// Lee mensajes desde stdin y los envia al usuario indicado.
fn procesar_entradas(socket: &UdpSocket, usuarios: Arc<Mutex<HashMap<String, SocketAddr>>>) {
    loop {
        println!("Escriba un mensaje:");

        let mut entrada = String::new();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer el mensaje");

        enviar_entrada(socket, &usuarios, entrada.trim());
    }
}

/// Interpreta una entrada y envia el mensaje a su destinatario.
fn enviar_entrada(
    socket: &UdpSocket,
    usuarios: &Arc<Mutex<HashMap<String, SocketAddr>>>,
    entrada: &str,
) {
    let mut partes = entrada.splitn(2, ' ');

    let destino = partes.next().unwrap_or("");
    let mensaje = partes.next().unwrap_or("");

    if destino.is_empty() || mensaje.is_empty() {
        println!("Formato esperado: <nickname> <mensaje>");
        return;
    }

    let usuarios = usuarios.lock().unwrap();

    if let Some(direccion) = usuarios.get(destino) {
        socket
            .send_to(mensaje.as_bytes(), direccion)
            .expect("Error al enviar el mensaje");
    } else {
        println!("Usuario {} no encontrado", destino);
    }
}

fn main() {
    let nickname = leer_nickname();
    let puerto = leer_puerto();

    let socket = crear_socket(puerto);

    let usuarios = Arc::new(Mutex::new(HashMap::new()));

    anunciar_nickname(&socket, &nickname, puerto);

    let socket_receptor = socket.try_clone().expect("No se pudo clonar el socket");

    let _receptor = iniciar_receptor(socket_receptor, Arc::clone(&usuarios));

    println!("Nickname: {}", nickname);
    println!("Puerto: {}", puerto);

    procesar_entradas(&socket, usuarios);
}
