use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Crea un worker que espera y ejecuta tareas del canal compartido.
fn crear_worker(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            let job = receiver.lock().unwrap().recv();

            match job {
                Ok(job) => job(),
                Err(_) => break,
            }
        }
    })
}

/// Crea un supervisor que reemplaza workers que terminan inesperadamente.
fn crear_supervisor(
    threads: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    stop_rx: mpsc::Receiver<()>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            if stop_rx.try_recv().is_ok() {
                break;
            }

            let mut workers = threads.lock().unwrap();

            for i in 0..workers.len() {
                if workers[i].is_finished() {
                    let nuevo_worker = crear_worker(Arc::clone(&receiver));

                    let worker_anterior = std::mem::replace(&mut workers[i], nuevo_worker);

                    if worker_anterior.join().is_err() {
                        println!("Un worker murió y fue reemplazado.");
                    }
                }
            }

            thread::sleep(std::time::Duration::from_millis(10));
        }
    })
}

/// Pool de threads que ejecuta tareas enviadas mediante un canal.
pub struct ThreadPool {
    threads: Arc<Mutex<Vec<thread::JoinHandle<()>>>>,
    sender: Option<mpsc::Sender<Job>>,
    stop_sender: Option<mpsc::Sender<()>>,
    supervisor: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Crea un nuevo thread pool con la cantidad indicada de workers.
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let (stop_tx, stop_rx) = mpsc::channel::<()>();
        let mut threads: Vec<thread::JoinHandle<()>> = Vec::new();

        for _ in 0..size {
            let receiver_clone = Arc::clone(&receiver);
            let handle = crear_worker(receiver_clone);
            threads.push(handle);
        }

        let threads = Arc::new(Mutex::new(threads));
        let supervisor = crear_supervisor(Arc::clone(&threads), Arc::clone(&receiver), stop_rx);

        ThreadPool {
            threads,
            sender: Some(sender),
            stop_sender: Some(stop_tx),
            supervisor: Some(supervisor),
        }
    }

    /// Envia una tarea al pool para ser ejecutada por un worker.
    pub fn spawn<F>(&self, tarea: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(tarea);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if let Some(stop_sender) = self.stop_sender.take() {
            let _ = stop_sender.send(());
        }

        if let Some(supervisor) = self.supervisor.take() {
            let _ = supervisor.join();
        }

        self.sender.take();

        let mut threads = self.threads.lock().unwrap();

        for handle in threads.drain(..) {
            if handle.join().is_err() {
                println!("Un thread terminó con error.");
            }
        }
    }
}
