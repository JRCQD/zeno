use std::collections::HashMap;

use crate::consumer::{ConsumerWorker, Pull};

type Subject = String;

pub struct ConsumerManager<'a, 's> {
    mmap: &'a [u8],
    subject_registry: HashMap<Subject, Box<[u8]>>,
    consumers: HashMap<Subject, Vec<ConsumerWorker<'s, Pull>>>,
}

impl<'a, 's> ConsumerManager<'a, 's>
where
    'a: 's,
{
    pub fn new(mmap: &'a [u8]) -> Self {
        ConsumerManager {
            mmap: mmap,
            consumers: HashMap::new(),
            subject_registry: HashMap::new(),
        }
    }

    pub fn register_new_subject(&mut self, subject: &[u8]) {
        // Assume always valid utf8 for now
        let str_subject = String::from_utf8(subject.to_vec()).unwrap();
        self.subject_registry
            .insert(str_subject.clone(), subject.to_vec().into_boxed_slice());
        self.consumers.insert(str_subject, Vec::new());
    }

    pub fn register_consumer(&'a mut self, subject: Subject) {
        let consumers = self.consumers.get_mut(&subject).unwrap();
        let subject = { &**self.subject_registry.get(&subject).unwrap() };
        let new_consumer = ConsumerWorker::new_pull(&self.mmap, subject, 0);
        consumers.push(new_consumer);
    }
}
