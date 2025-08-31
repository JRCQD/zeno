use std::{io::Read, net::{TcpListener, TcpStream}, thread, sync::Arc};
use crate::wal::WriteAheadLog;

use zeno_proto::publish::Message;
pub struct TcpIngress {
    listener: TcpListener
}

impl TcpIngress {
    pub fn new(connection: String) -> Self {
        let listener = TcpListener::bind(connection).unwrap();
        TcpIngress { listener }
    }

    pub fn listen(self, log: Arc<WriteAheadLog>) 
    {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let log = Arc::clone(&log);
            thread::spawn(move || {
                handle_connection(stream, log); 
            });
        }
    }
}


fn handle_connection(mut stream: TcpStream, log: Arc<WriteAheadLog>) {
    let mut buf = vec![0u8; 1024 * 1024];
    let mut pending = Vec::new();

    loop {
        match stream.read(&mut buf) {
            Ok(0) => break, // connection closed
            Ok(n) => {
                pending.extend_from_slice(&buf[..n]);

                let mut offset = 0;
                while offset + 5 <= pending.len() {
                    let subject_len = pending[offset] as usize;
                    let payload_len =
                        u32::from_le_bytes(pending[offset + 1..offset + 5].try_into().unwrap())
                            as usize;

                    let total_len = 5 + subject_len + payload_len;

                    if offset + total_len > pending.len() {
                        break;
                    }

                    let subject =
                        &pending[offset + 5..offset + 5 + subject_len];
                    let payload =
                        &pending[offset + 5 + subject_len..offset + total_len];

                    let msg = Message { subject, payload };
                    log.write_new_message(msg).unwrap();

                    offset += total_len;
                }

                pending.drain(..offset);
            }
            Err(e) => {
                eprintln!("error reading from stream: {e}");
                break;
            }
        }
    }
}
