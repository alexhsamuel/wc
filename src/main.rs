// Testing üü

use clap::{App, Arg};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Counts {
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
}

const EMPTY_COUNTS: Counts = Counts {
    bytes: 0,
    chars: 0,
    words: 0,
    lines: 0,
};

fn char_count(string: &str) -> usize {
    string.chars().count()
}

fn word_count(string: &str) -> usize {
    string.split_whitespace().count()
}

fn count_file(file: &File) -> Counts {
    let mut counts = Counts { ..EMPTY_COUNTS };

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
            counts.chars += char_count(&line);
            counts.words += word_count(&line);
            counts.lines += 1;
        }
    }

    counts
}

fn print_counts(path: &str, counts: &Counts, show_lines: bool, show_words: bool, show_chars: bool, show_bytes: bool) {
    if show_lines {
        print!("{:8} ", counts.lines);
    }
    if show_words {
        print!("{:8} ", counts.words);
    }
    if show_chars {
        print!("{:8} ", counts.chars);
    }
    if show_bytes {
        print!("{:8} ", counts.bytes);
    }
    println!("{}", path);
}    

fn main() {
    let exe = std::env::args().next().unwrap();
    let opts = App::new("word count")
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Prints byte counts"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Prints char counts"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Prints word counts"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Prints newline counts"),
        )
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .required(true)
                .multiple(true),
        )
        .get_matches();
    let show_bytes = opts.is_present("bytes");
    let mut show_chars = opts.is_present("chars");
    let mut show_words = opts.is_present("words");
    let mut show_lines = opts.is_present("lines");
    if !(show_bytes || show_chars || show_words || show_lines) {
        show_chars = true;
        show_words = true;
        show_lines = true;
    }

    for path in opts.values_of("path").unwrap() {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => {
                eprintln!("{}: {}: {}", exe, path, why);
                std::process::exit(1);
            }
        };
        let counts = count_file(&file);
        print_counts(&path, &counts, show_lines, show_words, show_chars, show_bytes);
    }
}
