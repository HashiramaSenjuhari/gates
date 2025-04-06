use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct GatesThread {
    sender: mpsc::Sender<Job>,
    // size: usize,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl GatesThread {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            let receiver = Arc::clone(&receiver);
            let worker = thread::spawn(move || {
                loop {
                    let recv = receiver.lock().unwrap().recv();
                    if let Ok(task) = recv {
                        // println!("{} {}", "running", i);
                        task();
                        // println!("{}", "completed");
                    } else {
                        break;
                    }
                }
            });
            workers.push(worker);
        }
        GatesThread {
            sender: tx,
            // size: size,
        }
    }
    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(task));
    }
}
