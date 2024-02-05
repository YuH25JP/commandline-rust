use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "number",
        help = "Number lines",
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank", help = "Number nonblank lines")]
    number_nonblank_lines: bool,

    #[arg(
        short = 'E',
        long = "show-ends",
        help = "display $ at end of each line"
    )]
    show_ends: bool,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(args: Args) -> MyResult<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("failed to open {}: {}", filename, e),
            Ok(br) => {
                // println!("Succesfully opend {}", filename);
                if !args.number_lines && !args.number_nonblank_lines {
                    for line in br.lines() {
                        println!("{}", line?);
                    }
                } else if args.number_lines {
                    for (idx, line) in br.lines().enumerate() {
                        println!("{:>6}\t{}", idx + 1, line.unwrap());
                    }
                } else {
                    let mut line_number = 1;
                    for line in br.lines() {
                        let line_string = line?;
                        if line_string == "".to_string() {
                            println!("{}", line_string);
                            continue;
                        } else {
                            println!("{:>6}\t{}", line_number, line_string);
                            line_number += 1;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
