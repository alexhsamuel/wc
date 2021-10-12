use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Counts {
    bytes: usize,
    words: usize,
    lines: usize,
}

const EMPTY_COUNTS: Counts = Counts {
    bytes: 0,
    words: 0,
    lines: 0,
};

fn word_count(string: &str) -> usize {
    string.split_whitespace().count()
}

fn main() {
    let exe = std::env::args().next().unwrap();
    let path = match std::env::args().skip(1).next() {
        Some(p) => p,
        None => panic!("no file given"),
    };

    let mut counts = Counts { ..EMPTY_COUNTS };

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => {
            eprintln!("{}: {}: {}", exe, path, why);
            std::process::exit(1);
        }
    };
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        line.clear();
        let read_len = reader.read_line(&mut line).unwrap();
        if read_len == 0 {
            // EOF.
            break;
        } else {
            counts.bytes += read_len;
            counts.words += word_count(&line);
            counts.lines += 1;
        }
    }

    println!(
        "{} {} {} {}",
        counts.lines, counts.words, counts.bytes, path
    );
}
