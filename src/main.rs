use clap::{App, Arg};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Counts {
    count: usize,
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
}

const EMPTY_COUNTS: Counts = Counts {
    count: 0,
    bytes: 0,
    chars: 0,
    words: 0,
    lines: 0,
};

fn print_error(msg: &str) {
    let exe = std::env::args().next().unwrap();
    eprintln!("{}: {}", exe, msg);
}

fn char_count(string: &str) -> usize {
    string.chars().count()
}

fn word_count(string: &str) -> usize {
    string.split_whitespace().count()
}

fn combine_counts(c0: &Counts, c1: &Counts) -> Counts {
    Counts {
        count: c0.count + c1.count,
        bytes: c0.bytes + c1.bytes,
        chars: c0.chars + c1.chars,
        words: c0.words + c1.words,
        lines: c0.lines + c1.lines,
    }
}

fn count_file(path: &str, file: &File) -> Counts {
    let mut counts = Counts {
        count: 1,
        ..EMPTY_COUNTS
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        line.clear();
        let read_len = match reader.read_line(&mut line) {
            Ok(len) => len,
            Err(why) => {
                print_error(&format!("{}: {}", path, why));
                return counts;
            }
        };
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

fn print_counts(
    path: &str,
    counts: &Counts,
    show_lines: bool,
    show_words: bool,
    show_chars: bool,
    show_bytes: bool,
) {
    if show_lines {
        print!("{:7} ", counts.lines);
    }
    if show_words {
        print!("{:7} ", counts.words);
    }
    if show_chars {
        print!("{:7} ", counts.chars);
    }
    if show_bytes {
        print!("{:7} ", counts.bytes);
    }
    println!("{}", path);
}

fn main() {
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

    let mut total = Counts { ..EMPTY_COUNTS };
    for path in opts.values_of("path").unwrap() {
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => {
                print_error(&format!("{}: {}", path, why));
                std::process::exit(1);
            }
        };
        let counts = count_file(&path, &file);
        print_counts(
            &path, &counts, show_lines, show_words, show_chars, show_bytes,
        );
        total = combine_counts(&total, &counts);
    }
    if total.count > 1 {
        print_counts(
            "total", &total, show_lines, show_words, show_chars, show_bytes,
        );
    }
}
