#[derive(Debug)]
pub enum CreateConsumerError {
    UnsupportedConsumerType{got: u8}
}

#[derive(Debug, PartialEq)]
pub enum ConsumerType {
    Pull
}

impl ConsumerType {
    fn from_bytes(byte: u8) -> Result<ConsumerType, CreateConsumerError> {
        match byte {
            0x1 => Ok(Self::Pull),
            _ => Err(CreateConsumerError::UnsupportedConsumerType { got: byte })
        }
    }
}

pub struct CreateConsumer<'a> {
    pub consumer_id: [u8; 16],
    pub subject: &'a [u8],
    pub consumer_type: ConsumerType
}

impl<'a> CreateConsumer<'a> {
    pub fn from_bytes(buf: &'a [u8]) -> Result<Self, CreateConsumerError> {
        let consumer_type = ConsumerType::from_bytes(buf[0])?;
        let mut consumer_name: [u8; 16] = [0x0; 16];
        consumer_name.copy_from_slice(&buf[1..17]);
        let subject_len: usize = buf[17].into();
        let subject = &buf[17+subject_len..];
        Ok(CreateConsumer{
            consumer_id: consumer_name,
            subject: subject,
            consumer_type
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::pull_wire_format::{ConsumerType, CreateConsumer};

    #[test]
    fn test_from_bytes() {
        let consumer = [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x41, 0x42];
        let consumer = CreateConsumer::from_bytes(&consumer).unwrap();
        assert_eq!(consumer.consumer_type, ConsumerType::Pull);
        assert_eq!(consumer.consumer_id, [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0]);
        assert_eq!(consumer.subject, [0x41, 0x42])
    }
}