use clap::{builder::PossibleValue, ArgAction, Parser, ValueEnum};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
enum EntryType {
    /// Directories
    Dir,
    /// Files
    File,
    /// Symbolic links
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(default_value = ".", help = "Search paths")]
    paths: Vec<String>,

    #[arg(short='n', long="name", value_parser(Regex::new), action(ArgAction::Append), num_args(0..), help="Name")]
    names: Vec<Regex>,

    #[arg(value_enum, short='t', long="type", value_parser(clap::value_parser!(EntryType)), action(ArgAction::Append), num_args(0..), help="Entry type")]
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    Ok(args)
}

pub fn run(args: Args) -> MyResult<()> {
    // println!("{:#?}", args);
    let re_vec = args.names;
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    let mut types = args.entry_types.clone();

                    // Regexでマッチ -----
                    let mut is_matched = false;
                    for re in &re_vec {
                        // let caps = re.captures(entry.file_name().to_str().unwrap());
                        let caps = re.is_match(&entry.file_name().to_string_lossy());
                        match caps {
                            false => {continue;},
                            true => is_matched = true,
                        }
                    }
                    // どのパターンにもマッチしなかった場合と，パターンが与えられていない場合は，次のentryへcontinue
                    if !is_matched && re_vec.clone().len() > 0 {
                        continue;
                    }

                    // -tオプションが与えられなかったら，すべてのタイプを対象にする
                    if types.is_empty() {
                        types.push(EntryType::Dir);
                        types.push(EntryType::File);
                        types.push(EntryType::Link);
                    }
                    let ft = entry.file_type();
                    let ft_res: EntryType;
                    // entryのファイルタイプを判定 -----
                    if ft.is_dir() {
                        ft_res = EntryType::Dir;
                    } else if ft.is_file() {
                        ft_res = EntryType::File;
                    } else {
                        ft_res = EntryType::Link;
                    }

                    // -tオプションで与えられたVecにentryのファイルタイプと一致するものがあればプリントする
                    if types.contains(&ft_res) {
                        println!("{}", entry.path().display());
                    }
                },
            }
        }
    }
    Ok(())
}
