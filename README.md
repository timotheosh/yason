# yason
Program that converts json to yaml, yaml to json. Written in Rust.

## Usage

``` shellsession
./target/debug/yason --help

Converts json to yaml or yaml to json

USAGE:
    yason [FLAGS] [OPTIONS] <infile>

FLAGS:
    -f, --force      Force overwrite if file exists.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --outfile <outfile>    Send output to file (use "-" for stdout). [default: -]

ARGS:
    <infile>    The file to convert.
```
