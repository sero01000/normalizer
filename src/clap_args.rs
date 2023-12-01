use super::hash_regex::HashType;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum WorkType {
    SpliterLimit(usize, usize),    // min, max
    LenLimit(usize, usize, usize), // item_id ,min, max
    ToLower(usize),                // item_id
    IsEmail(usize),                // item_id
    IsHash(usize),                 // item_id
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct UserInput {
    /// Path to .txt files
    #[arg(last = true, num_args = 1.., required=true, value_hint = clap::ValueHint::FilePath)]
    pub files: Vec<PathBuf>,

    /// Custom config, if not set use default
    #[clap(long, short, default_value = "false")]
    config: bool,

    /// Prefix for result files
    #[clap(long, short, default_value = "result")]
    prefix: String,

    /// List of spliters
    #[clap(long, short, default_value = ":,:", value_delimiter = ',')]
    spliters: Vec<String>,

    /// Split values minimum
    #[clap(long)]
    split_min: Option<usize>,

    /// Split values maximum
    #[clap(long)]
    split_max: Option<usize>,

    /// Index of value
    #[clap(long, short)]
    len_index: Option<Vec<usize>>,

    /// Limit value by min len
    #[clap(long, default_value = "5")]
    len_min: Vec<usize>,

    /// Limit value by max len
    #[clap(long, default_value = "40")]
    len_max: Vec<usize>,

    /// Value index to lower
    #[clap(long, short = 'o')]
    to_lower_index: Option<Vec<usize>>,

    /// Checking that a variable is a email
    #[clap(long, short)]
    email_index: Option<Vec<usize>>,

    /// Checking that a variable is not hash
    #[clap(long, short = 'a')]
    hash_index: Option<Vec<usize>>,

    /// Select hash types
    #[clap(
        long,
        short = 't',
        default_value = "half-md5,md5,sha1,sha224,sha256,sha384,sha512,half-md5-base64,md5-base64,sha1-base64,sha224-base64,sha256-base64,sha384-base64,sha512-base64,django-sha1,django-sha256,authme,md5-rus,drupal7,sha512crypt,blake2b-512,md5crypt,phpass,bcrypt,mysql5",
        value_delimiter = ','
    )]
    hash_types: Vec<HashType>,
}

pub fn get_config_2(input: &UserInput) -> Vec<WorkType> {
    match input.config {
        true => {
            let mut result: Vec<WorkType> = Vec::new();

            if let Some(min) = input.split_min {
                result.push(WorkType::SpliterLimit(
                    min,
                    input.split_max.unwrap_or(4294967294),
                ));
            }

            let len_min_default = input.len_min.first().unwrap_or(&0);
            let len_max_default = input.len_max.first().unwrap_or(&usize::MAX);
            let len_indexes = input.len_index.clone().unwrap_or_default();
            for num in 0..len_indexes.len() {
                result.push(WorkType::LenLimit(
                    len_indexes[num],
                    *input.len_min.get(num).unwrap_or(len_min_default),
                    *input.len_max.get(num).unwrap_or(len_max_default),
                ))
            }

            for index in input.to_lower_index.clone().unwrap_or_default() {
                result.push(WorkType::ToLower(index))
            }

            for index in input.email_index.clone().unwrap_or_default() {
                result.push(WorkType::IsEmail(index))
            }

            for index in input.hash_index.clone().unwrap_or_default() {
                result.push(WorkType::IsHash(index))
            }
            result
        }
        false => {
            vec![
                WorkType::SpliterLimit(2, 3),
                WorkType::LenLimit(0, 5, 40),  //user
                WorkType::LenLimit(1, 5, 137), //passw
                WorkType::ToLower(0),          //user
                WorkType::IsEmail(0),          //user
                WorkType::IsHash(1),           //passw
            ]
        }
    }
}
