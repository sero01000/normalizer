mod clap_args;
mod hash_regex;

use bytelines::*;
use clap::Parser;
use clap_args::{get_config_2, UserInput, WorkType};
use hash_regex::{hash_types, HashType, MAIL_REGEX};
// use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, ErrorKind, Write};
use std::path::PathBuf;

static PREFIX: &str = "result";
static SPLITERS: [&str; 2] = [":", ";"];

fn spliter_find(line: &str) -> Option<&str> {
    for spliter in SPLITERS {
        match line.contains(spliter) {
            true => return Some(spliter),
            false => continue,
        }
    }
    None
}

fn do_work(file_path: &PathBuf, work_types: &Vec<WorkType>) -> std::io::Result<()> {
    let path = std::path::Path::new(file_path);
    match path.is_file() {
        true => match path.extension().unwrap().to_string_lossy().as_ref() {
            "txt" => (),
            _ => return Err(std::io::Error::new(ErrorKind::Other, "not txt")),
        },
        false => return Err(std::io::Error::new(ErrorKind::Other, "is dir")),
    }
    let file = File::open(path)?;

    let folder_parent = path.parent().unwrap();
    let filename = path.file_stem().unwrap().to_string_lossy();

    let hash_types = hash_types();

    let file_good = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(folder_parent.join(format!("{}_{}_good.txt", &filename, &PREFIX,)))?;

    let mut bad_files: HashMap<Option<String>, File> = HashMap::new();
    bad_files.insert(None, file_good);

    let reader = BufReader::new(file);
    let lines = ByteLines::new(reader);

    for line in lines.into_iter() {
        match line {
            Ok(line) => {
                let mut line = String::from(String::from_utf8_lossy(&line));
                // let spliter = spliter_find(&line).unwrap_or("");
                let mut items = match spliter_find(&line){
                    Some(spliter)=>{
                        line.split(spliter).map(str::to_string).collect()
                    },
                    None=>{
                        vec![line.clone()]
                    }
                };
                // let mut items: Vec<String> = line.split(spliter).map(str::to_string).collect();

                let mut bad_type: Option<String> = None;
                for work_type in work_types {
                    match work_type {
                        WorkType::SpliterLimit(min, max) => {
                            let items_len = items.len();

                            if min <= &items_len && &items_len <= max {
                                continue;
                            } else {
                                bad_type = Some("split_limiter".to_string());
                                break;
                            }
                        }
                        WorkType::LenLimit(item_id, min, max) => {
                            let value = &items[*item_id];
                            let value_len = value.len();

                            if min <= &value_len && &value_len <= max {
                                continue;
                            } else {
                                bad_type = Some("len_limit".to_string());
                                break;
                            }
                        }
                        WorkType::ToLower(item_id) => {
                            let value = &items[*item_id];
                            items[*item_id] = value.to_lowercase();
                            line = items.join(":");
                        }
                        WorkType::IsEmail(item_id) => {
                            let value = &items[*item_id];
                            match MAIL_REGEX.is_match(value) {
                                true => {}
                                false => {
                                    bad_type = Some("not_email".to_string());
                                    break;
                                }
                            }
                        }
                        WorkType::IsHash(item_id) => {
                            let value = &items[*item_id];
                            let len_val = value.len();

                            match hash_types.get(&len_val) {
                                Some(hash_types) => {
                                    for hash_type in hash_types {
                                        match hash_type.regex.is_match(value) {
                                            true => {
                                                let mut new_bad_type = hash_type.hc.clone();
                                                if items.len() > 2 {
                                                    new_bad_type.push_str("_[SALT]");
                                                }
                                                bad_type = Some(new_bad_type);
                                                break; // to bad.txt
                                            }
                                            false => (), // not_match, continue
                                        }
                                    }
                                }
                                None => {
                                    continue; //value not hash
                                }
                            }
                        }
                    }
                }
                match bad_files.get(&bad_type) {
                    Some(mut bad_file) => {
                        line.push('\n');
                        bad_file.write_all(line.as_bytes())?;
                    }
                    None => {
                        //good file
                        let mut bad_file = std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(folder_parent.join(format!(
                                "{}_{}_{}_bad.txt",
                                &filename,
                                &PREFIX,
                                bad_type.clone().unwrap_or("none".to_string()),
                            )))?;
                        line.push('\n');
                        bad_file.write_all(line.as_bytes())?;
                        bad_files.insert(bad_type, bad_file);
                    }
                }
            }
            Err(e) => {
                println!("err_line:{:?}", (e, &filename))
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
pub enum WorkTypeClap {
    /// Limit by amount of spliter in string
    SpliterLimit {
        /// Minimum length
        #[clap(long, default_value = "2")]
        min: usize,

        /// Maximum length
        #[clap(long, default_value = "3")]
        max: usize,
    },
    /// Limit by value length
    LenLimit {
        #[clap(short, long)]
        item_id: usize,

        #[clap(long, default_value = "5")]
        min: usize,

        #[clap(long, default_value = "40")]
        max: usize,
    },
    /// Value to lower
    ToLower {
        #[clap(short, long)]
        item_id: usize,
    },
    /// Check is the value is email
    IsEmail {
        #[clap(short, long)]
        item_id: usize,
    },
    /// Check is the value is hash
    IsHash {
        #[clap(short, long)]
        item_id: usize,
        #[clap(long)]
        hash_types: Vec<HashType>,
        #[clap(long, default_value = "true")]
        hex_hashes: bool,
        #[clap(long, default_value = "true")]
        base64_hashes: bool,
        #[clap(long, default_value = "true")]
        hard_hashes: bool,
    },
}

fn main() -> std::io::Result<()> {
    let args = UserInput::parse();
    println!("{:?}", args);
    let config = get_config_2(&args);
    println!("{:?}", config);

    // args.files.par_iter().for_each(|file| {
    //     match do_work(file, &config) {
    //         Ok(_) => {}
    //         Err(e) => {
    //             println!("err:{:?} {:?}", e, file);
    //         }
    //     }
    // });
    do_work(&args.files[0], &config)?;

    Ok(())
}
