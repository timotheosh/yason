
use clap::{App, Arg};
use std::process;
use std::error::Error;
use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;

fn file_to_string(file_path: String) -> String {
    // Create a path to the desired file
    let path = Path::new(&file_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => s,
    }
}

fn json_to_yaml(file_path: String) -> String {
    let json= file_to_string(file_path);
    let data: serde_json::Result<serde_json::Value> = serde_json::from_str(&json);
    let data = match data {
        Ok(map) => map,
        Err(error) => {
            panic!("Problem reading json file: {:?} (is it valid json?)", error)
        },
    };
    serde_yaml::to_string(&data).unwrap()
}

fn yaml_to_json(file_path: String) -> String {
    let yaml: String = file_to_string(file_path);
    let data: serde_yaml::Result<serde_yaml::Value> = serde_yaml::from_str(&yaml);
    let data = match data {
        Ok(map) => map,
        Err(error) => {
            panic!("Problem reading yaml file: {:?} (is it valid yaml?)", error)
        },
    };
    serde_json::to_string_pretty(&data).unwrap()
}

fn convert(file_path: String) -> String {
    let path = Path::new(&file_path);
    let extension = path.extension();
    if extension == Some(OsStr::new("yml")) {
        yaml_to_json(file_path)
    } else if extension == Some(OsStr::new("json")) {
        json_to_yaml(file_path)
    }else {
        String::from("false")
    }
}

fn can_i_write_this_file(file_path: &str, force: bool) -> bool {
    match metadata(file_path) {
        Ok(f) => if force && f.is_file() && !f.permissions().readonly() {
            true
        } else {
            false
        },
        Err(_err) => true,
    }
}

fn write_to_file(data: String, file_path: String, force: bool) {
    if can_i_write_this_file(&file_path, force) {
        let mut f = File::create(&file_path).unwrap();
        f.write_all(data.as_bytes()).unwrap();
    } else {
        panic!("Cannot write to {}! If the file already exists, make sure it is a file, and it is not readonly, and use the force (-f).", file_path)
    }
}

fn main() {
    let matches = App::new("yason")
        .version("0.1")
        .author("Tim Hawes <thawes@gmail.com>")
        .about("Converts json to yaml or yaml to json")
        .arg(Arg::with_name("outfile")
            .short("o")
            .long("outfile")
            .takes_value(true)
            .help("Send output to file (use \"-\" for stdout).")
            .default_value("-"))
        .arg(Arg::with_name("force")
                 .short("f")
                 .long("force")
                 .help("Force overwrite if file exists.")
                 .takes_value(false))
        .arg(Arg::with_name("infile")
            .required(true)
            .help("The file to convert."))
        .get_matches();
    let outdata: String = convert(matches.value_of("infile").unwrap()
        .to_string());
    let outfile = matches.value_of("outfile").unwrap().to_string();
    let force: bool = matches.is_present("force");
    if outfile.eq("-") {
        println!("{}", outdata);
    } else {
        write_to_file(outdata, outfile, force);
    }
    process::exit(0);
}
