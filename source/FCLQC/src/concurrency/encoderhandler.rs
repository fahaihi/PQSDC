use crate::encoder::Encoder;
use crate::file_management;
use crate::lookuptable::LookUpTable;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum Message {
    Work(usize),
    End,
}

pub struct EncoderHandler {
    encoders: Vec<EncoderWorker>,  // Handles of encoders
    sender: mpsc::Sender<Message>, // Sender that gives the command to the thread
}

impl EncoderHandler {
    pub fn create_encoders_order(
        thread_num: usize,
        precision: u8,
        lut: &LookUpTable,
        name: &'static str,
    ) -> EncoderHandler {
        /*
           Creates threads of encoders and senders.

            Input:
                thread_num: number of threads to use
                precision: a fixed limit of number at that encoder rounds the calculated fractions to their nearest equivalents
                lut: probabilities look up table
                infile: subfiles of quality score
                infile_format: format of subfiles of quality score
                encoded_name: encoded file
                encoded_format: format of encoded file
            Output:
                 DecoderHandler: handler of encoder threads that encode quality scores and sender threads that execute the function encode
                 and sender that gives the command to encode qulaity scores
        */

        // create channels beteween main thread and sub threads
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut encoders = Vec::with_capacity(thread_num);

        for _ in 1..thread_num + 1 {
            encoders.push(EncoderWorker::new_encoders_order(
                precision,
                Arc::clone(&receiver),
                lut.clone(),
                name,
            ));
        }

        EncoderHandler { encoders, sender }
    }

    pub fn go_encode(&self, file_idx: usize) {
        /*
           Provide index of file to which current quality scores belong
           Input:
                file_idx: index of file to encode
        */

        self.sender.send(Message::Work(file_idx)).unwrap();
    }
}

impl Drop for EncoderHandler {
    fn drop(&mut self) {
        /*
           Drop the thread if the worker finished its job
        */

        for _ in &mut self.encoders {
            self.sender.send(Message::End).unwrap();
        }

        for worker in &mut self.encoders {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct EncoderWorker {
    thread: Option<thread::JoinHandle<()>>, // handle of encoder thread
}

impl EncoderWorker {
    fn new_encoders_order(
        precision: u8,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        lut: LookUpTable,
        name: &'static str,
    ) -> EncoderWorker {
        /*
            Create a encoder, that encodes quality scores, in a thread

            Input:
                precision: a fixed limit of number at that encoder rounds the calculated fractions to their nearest equivalents
                receiver: eceiver which listens the command of handler
                lut: probabilities look up table
                infile: name of divided FASTQ SUB-files
                infile_format: format of divided FASTQ SUB-files
                encoded_name: name of encoded file
                encoded_format: format of encoded file
            Output:
                EncoderWorker: handle of decoder
        */

        let mut encoder = Encoder::new(precision, &lut);

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::Work(file_index) => {
                    // create name of divided file to be opened and output file.
                    let infile_name = file_management::naming(name, "qs", file_index);
                    let outfile_name = file_management::naming(name, "enc", file_index);
                    encoder.encode(&infile_name, &outfile_name);
                }
                Message::End => break,
            }
        });

        EncoderWorker {
            thread: Some(thread),
        }
    }
}
