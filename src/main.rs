use clap::{App, Arg};
use std::cmp::max;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Read};

#[derive(Default)]
struct Counts {
    count: usize,
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
    maxln: usize,
}

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
        maxln: max(c0.maxln, c1.maxln),
    }
}

fn count<R: Read>(path: &str, read: R) -> Counts {
    let mut counts = Counts {
        count: 1,
        ..Default::default()
    };

    let mut reader = BufReader::new(read);
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
            counts.maxln = max(counts.maxln, char_count(line.trim_end()));
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
    show_maxln: bool,
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
    if show_maxln {
        print!("{:7} ", counts.maxln);
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
            Arg::with_name("maxln")
                .short("L")
                .long("max-line-length")
                .help("Prints the maximum line length"),
        )
        .arg(Arg::with_name("path").value_name("PATH").multiple(true))
        .get_matches();

    let show_bytes = opts.is_present("bytes");
    let mut show_chars = opts.is_present("chars");
    let mut show_words = opts.is_present("words");
    let mut show_lines = opts.is_present("lines");
    let show_maxln = opts.is_present("maxln");
    if !(show_bytes || show_chars || show_words || show_lines || show_maxln) {
        show_chars = true;
        show_words = true;
        show_lines = true;
    }
    let print_row = |p: &str, c: &Counts| {
        print_counts(
            p, c, show_lines, show_words, show_chars, show_bytes, show_maxln,
        )
    };

    let paths = match opts.values_of("path") {
        Some(vals) => vals.collect::<Vec<_>>(),
        None => vec!["-"],
    };

    let mut total: Counts = Default::default();
    for path in paths {
        let counts = if path == "-" {
            count("-", stdin())
        } else {
            let file = match File::open(&path) {
                Ok(file) => file,
                Err(why) => {
                    print_error(&format!("{}: {}", path, why));
                    std::process::exit(1);
                }
            };
            count(path, file)
        };
        print_row(path, &counts);
        total = combine_counts(&total, &counts);
    }
    if total.count > 1 {
        print_row("total", &total);
    }
}
