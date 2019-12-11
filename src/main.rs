
use std::env;
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ffi::OsStr;

fn usage(program_name: String) {
    println!("Usage: {} <json_file>  OR {} <yaml_file>", program_name, program_name);
    println!("\tWill convert either a json file to yaml or a yaml file to json, depending on input.");
}

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(args[0].clone());
        process::exit(1);
    } else {
        println!("{}", convert(args[1].clone()));
    }
    process::exit(0);
}
