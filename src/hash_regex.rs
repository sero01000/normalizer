use clap::ValueEnum;
use lazy_regex::{lazy_regex, Lazy, Regex};
use std::collections::HashMap;

pub static MAIL_REGEX: Lazy<Regex> =
    lazy_regex!(r"^\b[A-Za-z0-9._+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}\b$"i);

#[derive(Debug)]
pub struct HashValue {
    pub name: HashType,
    pub regex: Lazy<Regex>,
    pub hc: String,
}

#[derive(Debug)]
pub struct HashValue2 {
    pub name: HashType,
    pub regex: Lazy<Regex>,
    pub hc: String,
    pub len: u8,
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum HashType {
    Half_Md5,
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,

    Half_Md5_Base64,
    Md5_Base64,
    Sha1_Base64,
    Sha224_Base64,
    Sha256_Base64,
    Sha384_Base64,
    Sha512_Base64,

    Django_sha1,
    Django_sha256,
    Authme,
    Md5Rus,
    DRUPAL7,
    SHA512CRYPT,
    Blake2b_512,
    MD5CRYPT,
    PHPASS,
    Bcrypt,
    MYSQL5,
    //TODO:
    // 27200 	Ruby on Rails Restful Auth (one round, no sitekey) 	3999d08db95797891ec77f07223ca81bf43e1be2:5dcc47b04c49d3c8e1b9e4ec367fddeed21b7b85
    // 2611 	vBulletin < v3.8.5 	16780ba78d2d5f02f3202901c1b6d975:568
    // 2711 	vBulletin >= v3.8.5 	bf366348c53ddcfbd16e63edfdd1eee6:181264250056774603641874043270
}

impl HashType {
    pub fn vec() -> Vec<HashType> {
        vec![
            HashType::Half_Md5,
            HashType::MD5,
            HashType::SHA1,
            HashType::SHA224,
            HashType::SHA256,
            HashType::SHA384,
            HashType::SHA512,

            HashType::Half_Md5_Base64,
            HashType::Md5_Base64,
            HashType::Sha1_Base64,
            HashType::Sha224_Base64,
            HashType::Sha256_Base64,
            HashType::Sha384_Base64,
            HashType::Sha512_Base64,

            HashType::Django_sha1,
            HashType::Django_sha256,
            HashType::Authme,
            HashType::Md5Rus,
            HashType::DRUPAL7,
            HashType::SHA512CRYPT,
            HashType::Blake2b_512,
            HashType::MD5CRYPT,
            HashType::PHPASS,
            HashType::Bcrypt,
            HashType::MYSQL5,
        ]
    }
}

impl std::fmt::Display for HashType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn hash_types() -> HashMap<usize, Vec<HashValue>> {
    let half_md5 = HashValue {
        name: HashType::Half_Md5,
        regex: lazy_regex!(r"^[a-f0-9]{16}$"i),
        hc: String::from("[HALF_MD5]_[5100]"),
    };
    let md5 = HashValue {
        name: HashType::MD5,
        regex: lazy_regex!(r"^[a-f0-9]{32}$"i),
        hc: String::from("[MD5]_[0]"),
    };
    let sha1 = HashValue {
        name: HashType::SHA1,
        regex: lazy_regex!(r"^[a-f0-9]{40}$"i),
        hc: String::from("[SHA1]_[100]"),
    };
    let sha224 = HashValue {
        name: HashType::SHA224,
        regex: lazy_regex!(r"^[a-f0-9]{56}$"i),
        hc: String::from("[SHA224]_[1300]"),
    };
    let sha256 = HashValue {
        name: HashType::SHA256,
        regex: lazy_regex!(r"^[a-f0-9]{64}$"i),
        hc: String::from("[SHA256]_[1400]"),
    };
    let sha384 = HashValue {
        name: HashType::SHA384,
        regex: lazy_regex!(r"^[a-f0-9]{96}$"i),
        hc: String::from("[SHA384]_[10800]"),
    };
    let sha512 = HashValue {
        name: HashType::SHA512,
        regex: lazy_regex!(r"^[a-f0-9]{128}$"i),
        hc: String::from("[SHA512]_[1700]"),
    };

    let half_md5_base64 = HashValue {
        name: HashType::Half_Md5_Base64,
        regex: lazy_regex!(r"^[-A-Za-z0-9+/]{11}=$"i),
        hc: String::from("[HALF_MD5_BASE64]_[5100_base64]"),
    };
    let md5_base64 = HashValue {
        name: HashType::Md5_Base64,
        regex: lazy_regex!(r"^[-A-Za-z0-9+/]{22}==$"i),
        hc: String::from("[MD5_BASE64]_[0_base64]"),
    };
    let sha1_base64 = HashValue {
        name: HashType::Sha1_Base64,
        regex: lazy_regex!(r"^[-A-Za-z0-9+/]{27}=$"i),
        hc: String::from("[SHA1_BASE64]_[100_base64]"),
    };
    let sha224_base64 = HashValue {
        name: HashType::Sha224_Base64,
        regex: lazy_regex!(r"^[-A-Za-z0-9+/]{38}==$"i),
        hc: String::from("[SHA224_BASE64]_[1300_base64]"),
    };
    let sha256_base64 = HashValue {
        name: HashType::Sha256_Base64,
        regex: lazy_regex!(r"^[-A-Za-z0-9+/]{43}=$"i),
        hc: String::from("[SHA256_BASE64]_[1400_base64]"),
    };
    let sha384_base64 = HashValue {
        name: HashType::Sha384_Base64,
        regex: lazy_regex!(r"^[-A-Za-z0-9+/]{64}$"i),
        hc: String::from("[SHA384_BASE64]_[10800_base64]"),
    };
    let sha512_base64 = HashValue {
        name: HashType::Sha512_Base64,
        regex: lazy_regex!(r"^[-A-Za-z0-9+/]{86}==$"i),
        hc: String::from("[SHA512_BASE64]_[1700_base64]"),
    };

    let django_sha1 = HashValue {
        name: HashType::Django_sha1,
        regex: lazy_regex!(r"^sha1\$[a-z0-9]+\$[a-f0-9]{40}$"i),
        hc: String::from("[DJANGO_SHA1]_[124]"),
    };
    let django_sha256 = HashValue {
        name: HashType::Django_sha256,
        regex: lazy_regex!(r"^pbkdf2_sha256\$[0-9]+\$[a-z0-9/.]+\$[a-z0-9/.]{43}=$"i),
        hc: String::from("[DJANGO_SHA256]_[10000]"),
    };
    let authme = HashValue {
        name: HashType::Authme,
        regex: lazy_regex!(
            r"^\$sha\$[a-z0-9]{1,16}\$([a-f0-9]{32}|[a-f0-9]{40}|[a-f0-9]{64}|[a-f0-9]{128}|[a-f0-9]{140})$"i
        ),
        hc: String::from("[AUTHME]_[20711]"),
    };
    let md5_rus = HashValue {
        name: HashType::Md5Rus,
        regex: lazy_regex!(r"[A-Za-z0-9._+-\\\(\)^|*&%$#!~{}]+[a-f0-9]{32}$"i),
        hc: String::from("[MD5_RUs]_[20]"),
    };
    let drupal7 = HashValue {
        name: HashType::DRUPAL7,
        regex: lazy_regex!(r"^\$S\$[a-z0-9\/.]{52}$"i),
        hc: String::from("[DRUPAL7]_[7900]"),
    };
    let sha512crypt = HashValue {
        name: HashType::SHA512CRYPT,
        regex: lazy_regex!(r"^\$6\$(rounds=[0-9]+\$)?[a-z0-9\/.]{0,16}\$[a-z0-9\/.]{86}$"i),
        hc: String::from("[SHA512CRYPT]_[1800]"),
    };
    let blake2b_512 = HashValue {
        name: HashType::Blake2b_512,
        regex: lazy_regex!(r"\$BLAKE2\$[a-f0-9]{128}"),
        hc: String::from("[BLAKE2b_512]_[600]"),
    };
    let md5crypt = HashValue {
        name: HashType::MD5CRYPT,
        regex: lazy_regex!(r"^\$1\$[a-z0-9\/.]{0,8}\$[a-z0-9\/.]{22}(:.*)?$"i),
        hc: String::from("[MD5CRYPT]_[500]"),
    };
    let phpass = HashValue {
        name: HashType::PHPASS,
        regex: lazy_regex!(r"^\$[PH]\$[a-z0-9\\/.]{31}$"i),
        hc: String::from("[PHPASS]_[400]"),
    };
    let bcrypt = HashValue {
        name: HashType::Bcrypt,
        regex: lazy_regex!(r"^(\$2[axy]|\$2)\$[0-9]{2}\$[a-z0-9\/.]{53}$"i),
        hc: String::from("[BCRYPT]_[3200]"),
    };
    let mysql5 = HashValue {
        name: HashType::MYSQL5,
        regex: lazy_regex!(r"^\*[A-F0-9]{40}$"i),
        hc: String::from("[MYSQL5]_[300]"),
    };

    let mut hash_regexes: HashMap<usize, Vec<HashValue>> = HashMap::new();
    hash_regexes.insert(12, vec![half_md5_base64]);
    hash_regexes.insert(16, vec![half_md5]);
    hash_regexes.insert(24, vec![md5_base64]);
    hash_regexes.insert(28, vec![sha1_base64]);
    hash_regexes.insert(32, vec![md5]);
    hash_regexes.insert(34, vec![phpass, md5crypt]);
    hash_regexes.insert(40, vec![sha1, md5_rus, sha224_base64]);
    hash_regexes.insert(41, vec![mysql5]);
    hash_regexes.insert(44, vec![sha256_base64]);
    hash_regexes.insert(51, vec![django_sha1]);
    hash_regexes.insert(55, vec![drupal7]);
    hash_regexes.insert(56, vec![sha224]);
    hash_regexes.insert(60, vec![bcrypt]);
    hash_regexes.insert(64, vec![sha256, sha384_base64]);
    hash_regexes.insert(77, vec![django_sha256]);
    hash_regexes.insert(86, vec![authme]);
    hash_regexes.insert(88, vec![sha512_base64]);
    hash_regexes.insert(96, vec![sha384]);
    hash_regexes.insert(98, vec![sha512crypt]);
    hash_regexes.insert(128, vec![sha512]);
    hash_regexes.insert(136, vec![blake2b_512]);

    hash_regexes
}