use crc::Crc;
use itertools::Itertools;
use std::sync::mpsc::channel;

static WORDS: &str = include_str!("../data/words_out.txt");

fn main() {
    let cpus = match num_cpus::get() {
        0..1 => 1,
        n => n - 1,
    };
    let target = std::env::args().nth(1).unwrap();
    let target = if let Some(target) = target.strip_prefix("0x") {
        u32::from_str_radix(&target, 16).unwrap()
    } else {
        target.parse().unwrap()
    };
    println!("target: 0x{:08x}", target);

    let p = Permutation::new();
    let p_chunked = p.chunks(400000);
    let mut iter = p_chunked.into_iter();

    let mut workers = Vec::with_capacity(cpus);
    let mut worker_sends = Vec::with_capacity(cpus);
    let (answer_send, answer_recv) = channel::<(usize, Option<String>)>();
    for id in 0..cpus {
        let (input_send, input_recv) = channel::<Vec<String>>();
        let answer_send = answer_send.clone();
        let handle = std::thread::spawn(move || {
            let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
            for chunk in input_recv {
                if chunk.is_empty() {
                    return; // some other worker found the answer
                }
                for input in chunk {
                    let mut digest = crc.digest();
                    digest.update(input.as_bytes());
                    if digest.finalize() == target {
                        let _ = answer_send.send((id, Some(input)));
                        return;
                    }
                }
                let _ = answer_send.send((id, None));
            }
        });
        workers.push(handle);
        let chunk = iter.next().unwrap().collect::<Vec<_>>();
        input_send.send(chunk).unwrap();
        worker_sends.push(input_send);
    }
    drop(answer_send);

    let mut found = false;
    let mut count = 0;
    for (id, result) in answer_recv {
        if !found {
            println!("{}", count);
        }
        count += 1;
        if let Some(result) = result {
            println!("found: {}", result);
            found = true;
        }
        if found {
            let _ = worker_sends[id].send(Vec::new());
        } else {
            let chunk = iter.next().unwrap().collect::<Vec<_>>();
            let _ = worker_sends[id].send(chunk);
        }
    }
    for worker in workers {
        let _ = worker.join();
    }
}

struct Permutation {
    /// All words, including one empty string at 0
    words: Vec<&'static str>,
    /// Current index
    i: usize,
}

impl Permutation {
    fn new() -> Self {
        let mut words = vec![""];
        words.extend(WORDS.split_whitespace());
        Self { words, i: 1 }
    }
}

impl Iterator for Permutation {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = String::new();
        let mut i = self.i;
        let l = self.words.len();

        // limit to 2 words
        // if self.i > l * l + 1 {
        //     return None;
        // }

        while i > 0 {
            let w = i % l;
            result.push_str(self.words[w]);
            i /= l;
        }
        self.i += 1;
        Some(result)
    }
}
