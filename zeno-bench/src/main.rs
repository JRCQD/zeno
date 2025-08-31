use std::time::Instant;
use zeno_proto::publish::{Message, MAX_PAYLOAD_SIZE, MAX_SUBJECT_SIZE};
use zeno_core::wal::WriteAheadLog; // adjust paths as needed
use rand::{rng, thread_rng, Rng};
use rand_distr::{Alphanumeric, StandardUniform};


fn main() {

    const NUM_MESSAGES: usize = 1_000_000;
    const MAX_THREADS: usize = 4;
    const MESSAGES_PER_THREAD: usize = NUM_MESSAGES / MAX_THREADS;
    const MESSAGE_SIZE: usize = 1_000;
    let start = Instant::now();

    use std::{sync::Arc, thread};

    
    let log = Arc::new(WriteAheadLog::new());

    let threads: Vec<_> = (0..MAX_THREADS).map(|_| {
        let log = Arc::clone(&log);
        thread::spawn(move || {
            for _ in 0..MESSAGES_PER_THREAD {
                let subject = b"foo.bar.baz";
                let payload = vec![42u8; MESSAGE_SIZE]; // 128 bytes
                let msg = Message { subject, payload: &payload };
                log.write_new_message(msg).unwrap();
            }
        })
    }).collect();
    
    for t in threads {
        t.join().unwrap();
    }

    let duration = start.elapsed();
    let seconds = duration.as_secs_f64();
    let throughput = NUM_MESSAGES as f64 / seconds;

    let subject = b"foo";
    let payload = vec![42u8; MESSAGE_SIZE]; // 10k bytes

    let bytes_written = NUM_MESSAGES * (5 + subject.len() + payload.len());
    let mb_per_sec = bytes_written as f64 / 1024.0 / 1024.0 / seconds;

    println!("Approx {:.2} MB/s", mb_per_sec);
    println!("Wrote {} messages in {:.3} seconds", NUM_MESSAGES, seconds);
    println!("Throughput: {:.0} messages/sec", throughput);
}



fn generate_random_message<R: Rng>(rng: &mut R, max_payload_size: usize) -> (Vec<u8>, Vec<u8>) {
    let subject_len = rng.gen_range(1..=MAX_SUBJECT_SIZE);
    let payload_len = rng.gen_range(1..=max_payload_size);

    let subject: Vec<u8> = rng.sample_iter(&Alphanumeric)
        .take(subject_len)
        .map(|c| c as u8)
        .collect();

    let payload: Vec<u8> = rng.sample_iter(StandardUniform)
        .take(payload_len)
        .collect();

    (subject, payload)
}
