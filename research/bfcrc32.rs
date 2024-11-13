use clap::{Parser, ValueEnum};
use crc::Crc;
use itertools::Itertools;
use std::{io::Write, sync::mpsc::channel};

static WORDS: &str = include_str!("output/words.txt");

#[derive(Debug, Clone, Parser)]
struct Cli {
    /// Input hash to attack. Can be in decimal or hexadecimal format (prefixed with 0x)
    input: String,

    /// If digits (0-9) can appear in the output
    #[clap(short('d'), long)]
    allow_digits: bool,

    /// If underscores (_) can appear in the output (no effect if casing is snake or kebab)
    #[clap(short('u'), long)]
    allow_underscore: bool,

    /// If minus (-) can appear in the output (no effect if casing is snake or kebab)
    #[clap(short('m'), long)]
    allow_minus: bool,

    /// Casing of the output
    #[clap(short('c'), long)]
    casing: Option<Casing>,

    /// (Exact) prefix of the output
    #[clap(short, long)]
    prefix: Option<String>,
    //
    // /// Disallow output to start with these prefixes (exact)
    // #[clap(short, long)]
    // blacklist: Option<String>,
    /// The starting position, delimited by comma
    #[clap(short, long)]
    start: Option<String>,
    // /// Maximum number of words in the output. Each digit and symbol is counted as one word
    // #[clap(short, long)]
    // limit: Option<usize>,
}

/// Target string format
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Casing {
    /// All words are capitalized
    #[default]
    Pascal,
    /// First word is lowercase, rest are capitalized
    Camel,
    /// All words are lowercase, and connected with underscores
    Snake,
    /// All words are lowercase, and connected with minus (hyphen)
    Kebab,
}

fn main() {
    let cli = Cli::parse();
    let target = match cli.input.strip_prefix("0x") {
        Some(t) => u32::from_str_radix(t, 16).unwrap(),
        None => cli.input.parse().unwrap(),
    };
    let cpus = match num_cpus::get() {
        0..1 => 1,
        n => n - 1,
    };
    println!("target: 0x{:08x}", target);

    let p = Permutation::new(&cli);
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
            print!("\r{}", count);
            let _ = std::io::stdout().flush();
        }
        count += 1;
        if let Some(result) = result {
            println!();
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
    /// Casing used to connect the words
    casing: Casing,
    /// Prefix of the output
    prefix: String,
    // /// Disallow output to start with these prefixes (exact)
    // blacklist: Vec<String>,
}

impl Permutation {
    fn new(cli: &Cli) -> Self {
        let mut words = vec![""];
        if cli.allow_digits {
            words.extend(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
        }
        let casing = cli.casing.unwrap_or_default();
        if casing == Casing::Pascal || casing == Casing::Camel {
            if cli.allow_underscore {
                words.push("_");
            }
            if cli.allow_minus {
                words.push("-");
            }
        }
        words.extend(WORDS.split_whitespace());
        if !words.iter().any(|w| w == &"stamina") {
            panic!("missing 'stamina' in the word list");
        }
        let i = if let Some(start) = &cli.start {
            // least significant word is in the front of the output
            let mut i = 0;
            for s in start.split(',').rev() {
                let w = words.iter().position(|w| *w == s).unwrap();
                i = i * words.len() + w;
            }
            println!("start: {}", i);
            i + 1
        } else {
            1
        };
        // let blacklist = match &cli.blacklist {
        //     Some(bl) => bl.split(',').map(|s| s.to_string()).collect(),
        //     None => Vec::new(),
        // };
        Self {
            words,
            i,
            casing: cli.casing.unwrap_or_default(),
            prefix: cli.prefix.as_ref().cloned().unwrap_or_default(),
            // blacklist,
        }
    }
}

impl Iterator for Permutation {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.prefix.clone();
        let mut i = self.i;
        let l = self.words.len();

        // limit to 2 words
        if self.i > l * l * l + 1 {
            return None;
        }

        while i > 0 {
            let w = i % l;
            let word = self.words[w];
            if !word.is_empty() {
                match self.casing {
                    Casing::Pascal => {
                        let mut chars = word.chars();
                        result.extend(chars.next().unwrap().to_uppercase());
                        result.extend(chars);
                    }
                    Casing::Camel => {
                        if result.is_empty() {
                            result.push_str(word);
                        } else {
                            let mut chars = word.chars();
                            result.extend(chars.next().unwrap().to_uppercase());
                            result.extend(chars);
                        }
                    }
                    Casing::Snake => {
                        if !result.is_empty() {
                            result.push('_');
                        }
                        result.push_str(word);
                    }
                    Casing::Kebab => {
                        if !result.is_empty() {
                            result.push('-');
                        }
                        result.push_str(word);
                    }
                }
            }
            i /= l;
        }
        self.i += 1;
        Some(result)
    }
}
