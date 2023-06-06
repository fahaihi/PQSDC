use crate::decoder::Decoder;
use crate::file_management;
use crate::lookuptable::LookUpTable;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Message {
    Work(usize, usize, usize, bool),
    End,
}

pub struct DecoderHandler {
    decoders: Vec<DecoderWorker>,  // Handles of decoders
    sender: mpsc::Sender<Message>, // Sender that gives the command to the thread
}

impl DecoderHandler {
    pub fn create_decoders(
        precision: u8,
        thread_num: usize,
        lut: &LookUpTable,
        name: &'static str,
        outname: &'static str,
    ) -> DecoderHandler {
        /*
           Creates threads of decoders and senders

            Input:
                precision: precision of encoder
                thread_num: number of threads to use
                lut: Look Up Table of probability
                name: file name
            Output:
                DecoderHandler: handler of decoder threads that decode compressed quality scores and sender threads that execute the function decode
                and sender that gives the command to decode the compressed qulaity scores
        */

        // create channels beteween main thread and sub threads
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut decoders = Vec::with_capacity(thread_num);

        for _idx in 1..thread_num + 1 {
            decoders.push(DecoderWorker::new_decoders(
                precision,
                Arc::clone(&receiver),
                lut.clone(),
                name,
                outname,
            ));
        }

        DecoderHandler { decoders, sender }
    }

    pub fn go_decode(
        &self,
        file_idx: usize,
        first_pos: usize,
        file_size: usize,
        random_access: bool,
    ) {
        /*
           Provide index and size of file to which current data belongs

           Input:
                file_idx: index of file to current quality scores belong
                file_size: size of file
        */

        self.sender
            .send(Message::Work(file_idx, first_pos, file_size, random_access))
            .unwrap();
    }
}

impl Drop for DecoderHandler {
    fn drop(&mut self) {
        /*
           Drop the thread if the worker finished its job
        */

        for _ in &mut self.decoders {
            self.sender.send(Message::End).unwrap();
        }

        for worker in &mut self.decoders {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct DecoderWorker {
    thread: Option<thread::JoinHandle<()>>, // handle of thread composed decoder
}

impl DecoderWorker {
    fn new_decoders(
        precision: u8,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        lut: LookUpTable,
        name: &'static str,
        outname: &'static str,
    ) -> DecoderWorker {
        /*
           Create a decoder, that decodes compressed quality scores , in a thread

            Input:
                precision: a fixed limit of number at that encoder rounds the calculated fractions to their nearest equivalents
                receiver: eceiver which listens the command of handler
                lut: probabilities look up table
                name: file name
            Output:
                DecoderWorker: handle of decoder thread
        */

        // create decoder
        let mut decoder = Decoder::new(precision, &lut);

        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::Work(file_index, first_line, file_size, random_access) => {
                    // create a subfile name, and output file name.
                    let infile = file_management::naming(name, "enc", file_index);
                    let outfile = file_management::naming(outname, "dec", file_index);
                    if !random_access {
                        decoder.decode(&infile, &outfile, file_size);
                    } else {
                        decoder.randomacess_decode(&infile, &outfile, first_line, file_size);
                    }
                }
                Message::End => break,
            }
        });

        DecoderWorker {
            thread: Some(thread),
        }
    }
}
