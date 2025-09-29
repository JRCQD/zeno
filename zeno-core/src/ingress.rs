
use std::{sync::{Arc, mpsc}, net::{TcpListener, TcpStream}, thread, io::Read};
use zeno_proto::{client_commands::ClientCommand};
use crate::wal::WriteAheadLog;

pub fn start_pool(addr: &str, wal: Arc<WriteAheadLog>, workers: usize) {
    let listener = TcpListener::bind(addr).unwrap();
    let mut senders = Vec::new();

    for _ in 0..workers {
        let (tx, rx) = mpsc::channel::<TcpStream>();
        senders.push(tx);
        let wal = Arc::clone(&wal);
        thread::spawn(move || worker_loop(rx, wal));
    };

    let mut next = 0;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        senders[next].send(stream).unwrap();
        next = (next + 1) % workers;
    };
}


fn worker_loop(rx: mpsc::Receiver<TcpStream>, log: Arc<WriteAheadLog>) {
    let mut connections = Vec::new();

    loop {
        // pick up new connections
        while let Ok(stream) = rx.try_recv() {
            connections.push(Connection::new(stream));
        }

        // service connections
        for conn in &mut connections {
            if let Err(e) = conn.read_and_process( &log) {
                eprintln!("connection closed: {e}");
            }
        }
    }
}

struct Connection {
    stream: TcpStream,
    pending: Vec<u8>,
}

impl Connection {
    fn new(stream: TcpStream) -> Self {
        Self { stream, pending: Vec::new() }
    }

    fn read_and_process(&mut self, wal: &Arc<WriteAheadLog>) -> std::io::Result<()> {
        let mut buf = [0u8; 4096];
        let n = self.stream.read(&mut buf)?;
        if n == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "closed"));
        }
        self.pending.extend_from_slice(&buf[..n]);

        let mut offset = 0;
        while let Ok((cmd, consumed)) = ClientCommand::try_parse(&self.pending[offset..]) {
            offset += consumed;

            handle_command(cmd, &wal);
        }

        if offset > 0 {
            self.pending.drain(..offset);
        }

        Ok(())
    }
}


fn handle_command(cmd: ClientCommand, wal: &Arc<WriteAheadLog>) {
    match cmd {
        ClientCommand::Publish(msg) => {
            wal.write_new_message(msg).unwrap();
        }
        ClientCommand::CreateConsumer(cc) => {
            println!("new consumer: {:?}", cc);
            // TODO: call ConsumerManager
        }
    }
}

