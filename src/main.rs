use std::path::PathBuf;
use structopt::StructOpt;
use toml::{Value};
use toml::map::Map;
use std::io::prelude::*;
use std::fs::File;

// Arguments for ulai
#[derive(StructOpt, Clone)]
struct Arguments {
    #[structopt(short, long, parse(from_os_str), help="Path of the ulai file for the app that will be installed.")]
    // Directory of ulai file to use
    ulai: PathBuf
}

// I don't understand what this does after the value parse so it go into its own function 
// https://stackoverflow.com/a/57947338
// what??
fn parse_toml_struct_from_string(file: String) -> toml::map::Map<std::string::String, toml::Value> {
    file.parse::<Value>().ok().and_then(|r| match r {
        Value::Table(table) => Some(table),
        _ => None
    }).unwrap_or(Map::new())
}

// Check table X exists and contains key Y
fn sanity_has_table_x_has_key_y(contents: &toml::map::Map<std::string::String, toml::Value>, x: &str, y: &str) -> bool {
    // Check toml structure contains table x at current level with key y
    if contents.contains_key(x) && contents[x].is_table() {
        // Create map from the table that's inside the table... what? 
        let content_table = contents[x].as_table().unwrap();
        // return true if table x contains key y
        return content_table.contains_key(y)
    }
    // Table x not found
    false
}

// Validate ulai file
fn sanity_ulai(path: PathBuf) -> bool {
    // Check that ulai path exists
    if PathBuf::from(&path).exists() {
        // Parse ulai file as one long string
        let mut file = File::open(path).expect("Could not open ulai file");
        let mut ulai = String::new();
        file.read_to_string(&mut ulai).expect("Could not read ulai file");
        // Parse the long string as a toml struct 
        let toml: toml::map::Map<std::string::String, toml::Value>  = parse_toml_struct_from_string(ulai);
        let has_pkgname: bool = sanity_has_table_x_has_key_y(&toml, "Metadata", "pkgname");
        println!("Package name provided: {}", has_pkgname);
        if has_pkgname {
            println!("Package name: {}", toml["Metadata"]["pkgname"]);
            true
        }
        else {
            false
        }
    }
    else {
        false
    }
}

// Sanity check arguments that have been passed
fn argparse(args: Arguments) -> bool {
    // Run checks for each argument that is passed
    let sane_ulai = sanity_ulai(args.ulai);
    // Did all checks pass?
    if sane_ulai {
        true
    } else {
        false
    }   
}

fn main() {
    let args = Arguments::from_args();
    let arg_sanity = argparse(args.clone());

    //println!("Directory of ulai file to use: {}", args.ulai.into_os_string().into_string().unwrap());
    println!("Sanity check passed: {}", arg_sanity);
}
