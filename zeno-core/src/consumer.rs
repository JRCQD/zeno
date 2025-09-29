use std::marker::PhantomData;

use zeno_proto::publish::Message;

pub enum ConsumerMode {
    Push,
    Pull,
}

pub struct Pull;
pub struct Push;

pub struct ConsumerWorker<'a, Mode> {
    subject: &'a [u8],
    mmap: &'a [u8],
    cursor: usize,
    _mode: PhantomData<Mode>,
}

impl<'a> ConsumerWorker<'a, Pull> {
    pub fn new_pull(mmap: &'a [u8], subject: &'a [u8], start_offset: usize) -> Self {
        ConsumerWorker {
            subject,
            mmap,
            cursor: start_offset,
            _mode: PhantomData,
        }
    }

    pub fn poll_batch(&mut self, max: usize) -> Vec<Message<'a>> {
        let mut output = Vec::with_capacity(max);
        while output.len() < max && self.cursor < self.mmap.len() {
            let buffer = &self.mmap[self.cursor..];
            match Message::from_bytes(buffer) {
                Ok(msg) => {
                    let total_len = 5 + msg.subject.len() + msg.payload.len();
                    self.cursor += total_len;
                    if msg.subject == self.subject {
                        output.push(msg);
                    }
                }
                Err(_) => break,
            }
        }
        output
    }

    pub fn poll(&mut self) -> Option<Message<'a>> {
        while self.cursor < self.mmap.len() {
            let buf = &self.mmap[self.cursor..];
            match Message::from_bytes(buf) {
                Ok(msg) => {
                    let total_len = 5 + msg.subject.len() + msg.payload.len();
                    self.cursor += total_len;
                    if msg.subject == self.subject {
                        return Some(msg);
                    } else {
                        return None;
                    }
                }
                Err(_) => {}
            }
        }
        None
    }

    pub fn into_push(self) -> ConsumerWorker<'a, Push> {
        ConsumerWorker {
            subject: self.subject,
            mmap: self.mmap,
            cursor: self.cursor,
            _mode: PhantomData,
        }
    }
}

impl<'a> ConsumerWorker<'a, Push> {
    pub fn handle_message(&mut self, _msg: &Message) {}
}
