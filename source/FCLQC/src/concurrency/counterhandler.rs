use crate::counter::Counter;
use crate::file_management;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
    Work(usize),
    End,
}

pub struct CounterHandler {
    counters: Vec<CounterWorker>,  // Handles of counters
    sender: mpsc::Sender<Message>, // Sender that gives the command to the thread
}

impl CounterHandler {
    pub fn create_counters(
        thread_num: usize,
        q_value: (usize, usize),
        name: &'static str,
    ) -> CounterHandler {
        /*
           Creates threads of counters and senders
            Input:
                thread_num: number of threads to use
                qvalue : minimum and maximum ascii values of quality values
                name: name of the run
            Output:
                CounterHandler: handler of threads having counter that counts the number of occurrences
                and sender that gives the command to count the number and Look Up Table of all threads
        */

        // create channel beteween main thread and sub threads
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut counters = Vec::with_capacity(thread_num);

        for _idx in 0..thread_num {
            counters.push(CounterWorker::new_counter(
                Arc::clone(&receiver),
                q_value,
                name,
            ));
        }

        CounterHandler { counters, sender }
    }

    pub fn go_count(&self, file_idx: usize) {
        /*
           Provide a line of quality values to any available worker
            Input:
                file_idx: index of sub qsfile
        */
        self.sender.send(Message::Work(file_idx)).unwrap();
    }
}

impl Drop for CounterHandler {
    fn drop(&mut self) {
        /*
            Drop the thread if the worker finished its job
        */

        for _ in &mut self.counters {
            self.sender.send(Message::End).unwrap();
        }

        for worker in &mut self.counters {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct CounterWorker {
    thread: Option<thread::JoinHandle<()>>, // handles of counter thread
}

impl CounterWorker {
    fn new_counter(
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        q_value: (usize, usize),
        name: &'static str,
    ) -> CounterWorker {
        /*
           Create a counter, that counts the number of occurrences of initial quality value(marginal)
           and the pair of quality values (conditional), in a thread
            Input:
                receiver: receiver which listens the command of handler
                qvalue : minimum and maximum ascii values of quality values
                name: name of the run
            Output:
                CounterWorker: worker that counts occurrences in each thread
        */
        let mut counter = Counter::new(q_value);

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Work(file_index) => {
                    let infile = file_management::naming(name, "qs", file_index);
                    let outfile = file_management::naming(name, "cnt", file_index);
                    counter.count(&infile, &outfile);
                }
                Message::End => break,
            }
        });

        CounterWorker {
            thread: Some(thread),
        }
    }
}
