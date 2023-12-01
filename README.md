# normalizer
txt files normalizer
```bash
#By default use all filters
#Example use:
#Default config:
normalizer -- file1.txt
#Custom config:
normalizer -c --split-min 1 --split-max 3 -l 0 --len-min 5 --len-max 33 -l 1 --len-min 7 --len-max 34 -o 0 -e 0 -a 1 -- file1.txt
```

```bash
Usage: normalizer [OPTIONS] -- <FILES>...

Arguments:
  <FILES>...  Path to .txt files

Options:
  -c, --config
          Custom config, if not set use default
  -p, --prefix <PREFIX>
          Prefix for result files [default: result]
  -s, --spliters <SPLITERS>
          List of spliters [default: :,:]
      --split-min <SPLIT_MIN>
          Split values minimum
      --split-max <SPLIT_MAX>
          Split values maximum
  -l, --len-index <LEN_INDEX>
          Index of value
      --len-min <LEN_MIN>
          Limit value by min len [default: 5]
      --len-max <LEN_MAX>
          Limit value by max len [default: 40]
  -o, --to-lower-index <TO_LOWER_INDEX>
          Value index to lower
  -e, --email-index <EMAIL_INDEX>
          Checking that a variable is a email
  -a, --hash-index <HASH_INDEX>
          Checking that a variable is not hash
  -t, --hash-types <HASH_TYPES>
          Select hash types [default: half-md5,md5,sha1,sha224,sha256,sha384,sha512,half-md5-base64,md5-base64,sha1-base64,sha224-base64,sha256-base64,sha384-base64,sha512-base64,django-sha1,django-sha256,authme,md5-rus,drupal7,sha512crypt,blake2b-512,md5crypt,phpass,bcrypt,mysql5] [possible values: half-md5, md5, sha1, sha224, sha256, sha384, sha512, half-md5-base64, md5-base64, sha1-base64, sha224-base64, sha256-base64, sha384-base64, sha512-base64, django-sha1, django-sha256, authme, md5-rus, drupal7, sha512crypt, blake2b-512, md5crypt, phpass, bcrypt, mysql5]
  -h, --help
          Print help
  -V, --version
          Print version

```
