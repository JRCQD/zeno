#[derive(Debug)]
pub enum CreateConsumerError {
    UnsupportedConsumerType{got: u8},
    SubjectTooLong{expected: usize, got: usize}
}

#[derive(Debug)]
pub enum MessageSerdeError {
    SubjectTooLong { expected: usize, got: usize},
    MessageTooLong{expected: usize, got: usize},
    IncompleteMessage{expected: usize, got: usize},
    UnknownOpCode{got: usize}
}

#[derive(Debug)]
pub enum WireError {
    CreateConsumerError(CreateConsumerError),
    MessageSerdeError(MessageSerdeError),
    IncompleteMessage
}

impl From<CreateConsumerError> for WireError {
    fn from(err: CreateConsumerError) -> Self {
        WireError::CreateConsumerError(err)
    }
}

impl From<MessageSerdeError> for WireError {
    fn from(err: MessageSerdeError) -> Self {
        WireError::MessageSerdeError(err)
    }
}
