use clap::Parser;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(default_value = "-", help = "Input file(s)")]
    in_file: String,

    #[arg(help = "Output file")]
    out_file: Option<String>,

    #[arg(
        short = 'c',
        long = "count",
        help = "prefix lines by the number of occurrences"
    )]
    count: bool,
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
    let mut file = open(&args.in_file).map_err(|e| format!("{}: {}", args.in_file, e))?;
    let mut line = String::new();
    let mut hm = HashMap::new();
    loop {
        let mut bytes = file.read_line(&mut line)?;
        if bytes == 0 { break; }

        let count = hm.entry(line.clone()).or_insert(0);
        *count += 1;

        line.clear();
    }
    for (k, v) in hm.into_iter() {
        if args.count {
            print!("{:>4} {}", v, k);
        } else {
            print!("{}", k);
        }
    }
    Ok(())
}
