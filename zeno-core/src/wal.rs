use memmap2::{Mmap, MmapMut};
use std::{
    fs::OpenOptions,
    io::Write,
    sync::atomic::{AtomicUsize, Ordering::Acquire},
    cell::UnsafeCell
};
use zeno_proto::publish::{Message, MessageSerdeError};
const WAL_CAPACITY: usize = 1024 * 1024 * 1024; // 1 GiB
#[derive(Debug)]
pub struct WriteAheadLog {
    memory_mapped_file: UnsafeCell<MmapMut>,
    cursor: AtomicUsize,
}

impl WriteAheadLog {
    pub fn new() -> Self {
        let file = Self::get_mmap_file();
        WriteAheadLog {
            memory_mapped_file: UnsafeCell::new(file),
            cursor: AtomicUsize::new(0),
        }
    }

    fn get_mmap_file() -> MmapMut {
        let log = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("log.log")
            .unwrap();
        log.set_len(WAL_CAPACITY as u64).unwrap();
        let mapped = unsafe { Mmap::map(&log).unwrap() };
        mapped.make_mut().unwrap()
    }

    pub fn write_new_message(&self, message: Message) -> Result<(), MessageSerdeError> {
        let mut new_message = Vec::new();
        message.to_bytes(&mut new_message)?;
        let next_chunk_start = self.cursor.fetch_add(new_message.len(), Acquire);
        let mmap = unsafe { &mut *self.memory_mapped_file.get() };
        (&mut mmap[next_chunk_start..next_chunk_start + new_message.len()])
        .write_all(&new_message)
        .unwrap();
        Ok(())
    }
}

unsafe impl Send for WriteAheadLog{}
unsafe impl Sync for WriteAheadLog{}
