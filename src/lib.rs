use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

type Job = Box<dyn FnOnce() -> () + Send + 'static>;

pub struct ThreadPool {
    sender: mpsc::Sender<Job>,
    // threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(count: usize) -> ThreadPool {
        assert!(count > 0);

        //let mut threads = Vec::with_capacity(count);

        let (sender, receiver) = mpsc::channel::<Job>();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..count {
            let receiver = Arc::clone(&receiver);

            thread::spawn(move || loop {
                let job = receiver.lock().unwrap().iter().next().unwrap();

                println!("Excuting job on thread #{}", id + 1);

                job();
            });

            // threads.push(new_thread);
        }

        ThreadPool { sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
