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

fn main() {
    let exe = std::env::args().next().unwrap();
    let opts = App::new("word count")
        .arg(Arg::with_name("bytes").short("c").long("bytes"))
        .arg(Arg::with_name("chars").short("m").long("chars"))
        .arg(Arg::with_name("words").short("w").long("words"))
        .arg(Arg::with_name("lines").short("l").long("lines"))
        .arg(Arg::with_name("path").value_name("PATH").required(true))
        .get_matches();
    let path = opts.value_of("path").unwrap();
    let show_bytes = opts.is_present("bytes");
    let mut show_chars = opts.is_present("chars");
    let mut show_words = opts.is_present("words");
    let mut show_lines = opts.is_present("lines");
    if !(show_bytes || show_chars || show_words || show_lines) {
        show_chars = true;
        show_words = true;
        show_lines = true;
    }

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
            counts.chars += char_count(&line);
            counts.words += word_count(&line);
            counts.lines += 1;
        }
    }

    if show_lines {
        print!("{} ", counts.lines);
    }
    if show_words {
        print!("{} ", counts.words);
    }
    if show_chars {
        print!("{} ", counts.chars);
    }
    if show_bytes {
        print!("{} ", counts.bytes);
    }
    println!("{}", path);
}
