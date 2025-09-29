use crate::{publish::Message, pull_wire_format::CreateConsumer, errors::{WireError, MessageSerdeError}};

#[derive(Debug, Copy, Clone)]
pub enum ClientCommand<'a> {
    Publish(Message<'a>),
    CreateConsumer(CreateConsumer<'a>)
}

impl<'a> ClientCommand<'a> {
    pub fn try_parse(buffer: &'a [u8]) -> Result<(Self, usize), WireError> {
        match buffer.get(0) {
            Some(0x1) => {
                let msg = Message::from_bytes(&buffer)?;
                let consumed = 1 + 5 + msg.subject.len() + msg.payload.len();
                Ok((ClientCommand::Publish(msg), consumed))
            },
            Some(0x2) => {
                let consumer = CreateConsumer::from_bytes(&buffer)?;
                let consumed = 1 + 16 + 1 + consumer.subject.len() + 1;
                Ok((ClientCommand::CreateConsumer(consumer), consumed))
            },
            Some(val) => Err(MessageSerdeError::UnknownOpCode{got: *val as usize}.into()),
            None => Err(MessageSerdeError::UnknownOpCode { got: 0x0 }.into()),
        }
    }
}

