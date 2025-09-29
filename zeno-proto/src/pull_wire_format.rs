use crate::publish::MAX_SUBJECT_SIZE;
use crate::errors::{WireError, CreateConsumerError};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConsumerType {
    Pull,
}

impl ConsumerType {
    fn from_bytes(byte: u8) -> Result<ConsumerType, WireError> {
        match byte {
            0x2 => Ok(Self::Pull),
            _ => Err(CreateConsumerError::UnsupportedConsumerType { got: byte }.into()),
        }
    }

    fn to_bytes(&self) -> u8 {
        match self {
            Self::Pull => 0x2,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CreateConsumer<'a> {
    pub consumer_id: [u8; 16],
    pub subject: &'a [u8],
    pub consumer_type: ConsumerType,
}

impl<'a> CreateConsumer<'a> {
    pub fn from_bytes(buf: &'a [u8]) -> Result<Self, WireError> {
        let consumer_type = ConsumerType::from_bytes(buf[0])?;
        let mut consumer_name: [u8; 16] = [0x0; 16];
        consumer_name.copy_from_slice(&buf[1..17]);
        let subject_len: usize = buf[17].into();
        println!("subject len: {}", subject_len);
        let subject = &buf[18..];
        Ok(CreateConsumer {
            consumer_id: consumer_name,
            subject: subject,
            consumer_type,
        })
    }

    pub fn to_bytes(&self, buf: &mut Vec<u8>) -> Result<(), WireError> {
        let consumer_type = self.consumer_type.to_bytes();
        if self.subject.len() > MAX_SUBJECT_SIZE {
            return Err(CreateConsumerError::SubjectTooLong {
                expected: MAX_SUBJECT_SIZE,
                got: self.subject.len(),
            }.into());
        }
        buf.push(consumer_type);
        buf.extend(self.consumer_id);
        buf.push(self.subject.len() as u8);
        buf.extend(self.subject);
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use crate::pull_wire_format::{ConsumerType, CreateConsumer};

    #[test]
    fn test_from_bytes() {
        let consumer = [
            0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x2, 0x41, 0x42,
        ];
        let consumer = CreateConsumer::from_bytes(&consumer).unwrap();
        assert_eq!(consumer.consumer_type, ConsumerType::Pull);
        assert_eq!(
            consumer.consumer_id,
            [
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
            ]
        );
        assert_eq!(consumer.subject, [0x41, 0x42])
    }

    #[test]
    fn test_to_bytes() {
        let consumer_array = [
            0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x2, 0x41, 0x42,
        ];
        let consumer = CreateConsumer::from_bytes(&consumer_array).unwrap();
        assert_eq!(consumer.consumer_type, ConsumerType::Pull);
        assert_eq!(
            consumer.consumer_id,
            [
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
            ]
        );
        assert_eq!(consumer.subject, [0x41, 0x42]);
        let mut buffer = Vec::new();
        consumer.to_bytes(&mut buffer).unwrap();

        assert_eq!(buffer, consumer_array.to_vec());
    }
}
