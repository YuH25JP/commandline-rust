use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(author = "YuH25JP <cppyuh@gmail.com>")]
#[command(about="wc command written in Rust", long_about = None)]
pub struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'l', long = "lines", help = "print the newline counts")]
    lines: bool,

    #[arg(short = 'w', long = "words", help = "print the word counts")]
    words: bool,

    #[arg(
        short = 'c',
        long = "bytes",
        conflicts_with("chars"),
        help = "print the character counts"
    )]
    bytes: bool,

    #[arg(short = 'm', long = "chars", help = "print the character counts")]
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    if !args.lines & !args.words & !args.bytes & !args.chars {
        // if no flags designated, turn `lines`, `words`, `bytes` into true
        let new_args = Args {
            files: args.files,
            lines: true,
            words: true,
            bytes: true,
            chars: false,
        };
        return Ok(new_args);
    }
    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // buffer from standard input
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // buffer from the file
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut buf = String::new();

    loop {
        let bytes = file.read_line(&mut buf)?;
        if bytes == 0 {
            break;
        }
        num_lines += 1;
        num_words += buf.split_whitespace().count();
        num_bytes += bytes;
        num_chars += buf.as_bytes().len();
        buf.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_result(flag: &bool, num: &usize) -> String {
    if flag == &true {
        format!("{:>8}", num)
    } else {
        "".to_string()
    }
}

pub fn run(args: Args) -> MyResult<()> {
    // println!("{:#?}", args);
    let num_files = args.files.len();

    // total of all given files
    let mut file_info_total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };

    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e), // does not panic even if failed to read a file
            Ok(br) => match count(br) {
                Err(e) => eprintln!("{}", e),
                Ok(file_info) => {
                    if filename == "-" {
                        println!(
                            "{:>8}{:>8}{:>8}",
                            file_info.num_lines, file_info.num_words, file_info.num_bytes
                        );
                    } else {
                        println!(
                            "{}{}{}{} {}",
                            format_result(&args.lines, &file_info.num_lines),
                            format_result(&args.words, &file_info.num_words),
                            format_result(&args.bytes, &file_info.num_bytes),
                            format_result(&args.chars, &file_info.num_chars),
                            filename
                        );
                    }
                    file_info_total.num_lines += file_info.num_lines;
                    file_info_total.num_words += file_info.num_words;
                    file_info_total.num_bytes += file_info.num_bytes;
                    file_info_total.num_chars += file_info.num_chars;
                }
            },
        }
    }

    if num_files >= 2 {
        println!(
            "{}{}{}{} total",
            format_result(&args.lines, &file_info_total.num_lines),
            format_result(&args.words, &file_info_total.num_words),
            format_result(&args.bytes, &file_info_total.num_bytes),
            format_result(&args.chars, &file_info_total.num_chars),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
