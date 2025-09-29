pub enum MessageTypes {
    Message,
    CreateConsumer,
    AckMessage,
    RemoveConsumer,
}

pub enum FrameError {
    UnknownFrameType { got: u8 },
}

impl MessageTypes {
    pub fn from_byte(frame_header: u8) -> Result<Self, FrameError> {
        match frame_header {
            0x1 => Ok(MessageTypes::Message),
            0x2 => Ok(MessageTypes::CreateConsumer),
            0x3 => Ok(MessageTypes::AckMessage),
            0x4 => Ok(MessageTypes::RemoveConsumer),
            _ => Err(FrameError::UnknownFrameType { got: frame_header }),
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            MessageTypes::Message => 0x1,
            MessageTypes::CreateConsumer => 0x2,
            MessageTypes::AckMessage => 0x3,
            MessageTypes::RemoveConsumer => 0x4,
        }
    }
}
