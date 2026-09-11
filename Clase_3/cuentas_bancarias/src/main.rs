mod account;

use account::Account;
use std::sync::{Arc, RwLock, mpsc};
use std::thread;

/// Crea un thread que realiza un deposito y se sincroniza mediante canales.
fn crear_cliente_deposito(
    account: Arc<RwLock<Account>>,
    amount: i32,
    receptor: Option<mpsc::Receiver<()>>,
    emisor: Option<mpsc::Sender<()>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if let Some(rx) = receptor {
            rx.recv().unwrap();
        }

        let mut cuenta = account.write().unwrap();
        cuenta.deposit(amount);
        drop(cuenta);

        if let Some(tx) = emisor {
            tx.send(()).unwrap();
        }
    })
}

/// Crea un thread que realiza una extraccion y se sincroniza mediante canales.
fn crear_cliente_extraccion(
    account: Arc<RwLock<Account>>,
    amount: i32,
    receptor: Option<mpsc::Receiver<()>>,
    emisor: Option<mpsc::Sender<()>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if let Some(rx) = receptor {
            rx.recv().unwrap();
        }

        let mut cuenta = account.write().unwrap();
        cuenta.withdraw(amount);
        drop(cuenta);

        if let Some(tx) = emisor {
            tx.send(()).unwrap();
        }
    })
}

/// Crea los threads de los clientes y configura su sincronizacion.
fn crear_threads(account: &Arc<RwLock<Account>>) -> Vec<thread::JoinHandle<()>> {
    let (tx_1, rx_1) = mpsc::channel();
    let (tx_2, rx_2) = mpsc::channel();
    let (tx_3, rx_3) = mpsc::channel();

    let customer1_handle = crear_cliente_deposito(Arc::clone(account), 40, None, Some(tx_1));

    let customer2_handle =
        crear_cliente_extraccion(Arc::clone(account), 30, Some(rx_1), Some(tx_2));

    let customer3_handle = crear_cliente_deposito(Arc::clone(account), 60, Some(rx_2), Some(tx_3));

    let customer4_handle = crear_cliente_extraccion(Arc::clone(account), 70, Some(rx_3), None);

    vec![
        customer1_handle,
        customer2_handle,
        customer3_handle,
        customer4_handle,
    ]
}

fn main() {
    let account = Arc::new(RwLock::new(Account::new(0)));

    let handles = crear_threads(&account);

    for handle in handles {
        handle.join().unwrap();
    }

    let savings = account.read().unwrap().balance();
    println!("Balance: {}", savings);
}
