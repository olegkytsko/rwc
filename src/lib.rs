use clap::builder::Command;
use clap::{Arg, ArgAction};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    chars: bool,
    words: bool,
    lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;

    let args = config.files;
    if args.is_empty() {
        eprintln!("Usage: {} text_file(s)", args[0]);
        std::process::exit(1)
    }

    for path in args.iter() {
        let file_open = File::open(path);

        match file_open {
            Err(err) => eprintln!("Failed to open {}: {}", path, err),
            Ok(f) => {
                let mut file = BufReader::new(f);
                let mut buffer = Vec::new();

                let mut lines_count: usize = 0;
                let mut words_count: usize = 0;
                let mut chars_count: usize = 0;

                let mut result_string = String::new();

                while file.read_until(b'\n', &mut buffer)? > 0 {
                    let line = String::from_utf8_lossy(&buffer);

                    if config.lines && line.contains('\n') {
                        lines_count += 1;
                    };

                    if config.words {
                        words_count += line.split_whitespace().count();
                    }

                    if config.chars {
                        chars_count += line.len();
                    }

                    buffer.clear();
                }

                if config.lines {
                    output_str(&mut result_string, lines_count);
                }
                if config.words {
                    output_str(&mut result_string, words_count);
                }
                if config.chars {
                    output_str(&mut result_string, chars_count);
                }

                println!("{} {}", result_string, path);

                total_lines += lines_count;
                total_words += words_count;
                total_chars += chars_count;
            }
        }
    }

    if args.len() > 1 {
        println!(
            "{:>8}{:>8}{:>8} total",
            total_lines, total_words, total_chars
        );
    }

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("rwc")
        .version("0.0.1")
        .author("Oleh Kytsko")
        .about("Rust wc")
        .arg(
            Arg::new("characters")
                .action(ArgAction::SetTrue)
                .short('m')
                .long("chars")
                .help(
                    "The number of characters in each input file is written to the
                standard output.",
                )
                .required(false),
        )
        .arg(
            Arg::new("words")
                .action(ArgAction::SetTrue)
                .short('w')
                .long("words")
                .help(
                    "The number of words in each input file is written to the standard
                output.",
                )
                .required(false),
        )
        .arg(
            Arg::new("lines")
                .action(ArgAction::SetTrue)
                .short('l')
                .long("lines")
                .help(
                    "The number of lines in each input file is written to the standard
                output.",
                )
                .required(false),
        )
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .value_name("FILE")
                .help("Input file(s)")
                .required(true),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("files")
        .unwrap()
        .cloned()
        .collect();

    let mut chars = matches.get_flag("characters");
    let mut words = matches.get_flag("words");
    let mut lines = matches.get_flag("lines");

    if !chars && !words && !lines {
        (chars, words, lines) = (true, true, true);
    }

    Ok(Config {
        files,
        chars,
        words,
        lines,
    })
}

fn output_str(input_str: &mut String, count: usize) {
    input_str.push_str(&format!("{:>8}", count));
}
