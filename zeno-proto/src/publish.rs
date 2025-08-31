pub const MAX_SUBJECT_SIZE: usize = 255;
pub const MAX_PAYLOAD_SIZE: usize = 1_048_576;

#[derive(Debug)]
pub enum MessageSerdeError {
    SubjectTooLong { expected: usize, got: usize },
    MessageTooLong { expected: usize, got: usize },
    IncompleteMessage { expected: usize, got: usize },
}

#[derive(Debug, Clone, Copy)]
pub struct Message<'a> {
    pub subject: &'a [u8],
    pub payload: &'a [u8],
}

impl<'a> Message<'a> {
    pub fn from_bytes(buf: &'a [u8]) -> Result<Self, MessageSerdeError> {
        if buf.len() < 5 {
            return Err(MessageSerdeError::IncompleteMessage {
                expected: 5,
                got: buf.len(),
            });
        }

        // first byte is the length of the subject
        let subject_len = buf[0] as usize;
        if subject_len > MAX_SUBJECT_SIZE {
            return Err(MessageSerdeError::SubjectTooLong {
                expected: MAX_SUBJECT_SIZE,
                got: subject_len,
            });
        }
        // next 4 bytes is the length of the message, capped at 1MB
        let msg_len = usize::from_le_bytes(buf[1..5].try_into().unwrap());
        if msg_len > MAX_PAYLOAD_SIZE {
            return Err(MessageSerdeError::MessageTooLong {
                expected: MAX_PAYLOAD_SIZE,
                got: msg_len,
            });
        }
        if 5 + subject_len + msg_len != buf.len() {
            return Err(MessageSerdeError::IncompleteMessage {
                expected: 5 + subject_len + msg_len,
                got: buf.len(),
            });
        }
        let subject = &buf[5..(5 + subject_len)];
        let message = &buf[(5 + subject_len)..buf.len()];
        Ok(Message {
            subject,
            payload: message,
        })
    }

    pub fn to_bytes(&self, buf: &mut Vec<u8>) -> Result<(), MessageSerdeError> {
        if self.subject.len() > MAX_SUBJECT_SIZE {
            return Err(MessageSerdeError::SubjectTooLong {
                expected: MAX_SUBJECT_SIZE,
                got: self.subject.len(),
            });
        }

        if self.payload.len() > MAX_PAYLOAD_SIZE {
            return Err(MessageSerdeError::MessageTooLong {
                expected: MAX_PAYLOAD_SIZE,
                got: self.payload.len(),
            });
        }
        buf.push(self.subject.len() as u8);
        buf.extend(&(self.payload.len() as u32).to_le_bytes());
        buf.extend(self.subject);
        buf.extend(self.payload);

        Ok(())
    }
}
