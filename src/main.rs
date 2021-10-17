use std::path::PathBuf;
use structopt::StructOpt;
use toml::{Value};
use toml::map::Map;
use std::fs::read_to_string;

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
fn parse_toml_struct_from_string(file: &String) -> toml::map::Map<std::string::String, toml::Value> {
    file.parse::<Value>().ok().and_then(|r| match r {
        Value::Table(table) => Some(table),
        _ => None
    }).unwrap_or(Map::new())
}

// Check table X exists and contains key Y
/*
fn sanity_has_table_x_has_key_y(contents: &toml::map::Map<std::string::String, toml::Value>, table: &str, key: &str) -> bool {
    // Check toml structure contains table x at current level with key y
    if contents.contains_key(table) && contents[table].is_table() {
        // Create map from the table that's inside the table... what? 
        let content_table = contents[table].as_table().unwrap();
        // return true if table x contains key y
        return content_table.contains_key(key)
    }
    // Table x not found
    false
}*/

// Check that this table (or sets of tables) exist within the toml struct
fn nth_dimension_is_table(toml: &toml::map::Map<std::string::String, toml::Value>, list_of_table_indexes: Vec<&str>) -> bool {
    let mut current_table_layer = toml;
    for element in list_of_table_indexes {
        if current_table_layer.contains_key(element) & current_table_layer[element].is_table() {
            current_table_layer = current_table_layer[element].as_table().unwrap();
            //println!("There is a table called {} in this toml", element);
        }
        else {
            return false
        }
    }
    true
}
// Wrapper for function above that splits the indexes passed as a vector for you
fn nth_dimension_is_table_stringin(toml: &toml::map::Map<std::string::String, toml::Value>, table_indexes: &str) -> bool {
    nth_dimension_is_table(toml, table_indexes.split(".").collect::<Vec<&str>>())
}

// Takes a set of indexes and confirms whether a value is there (NOT A TABLE!) by iterating through each table and then the value itself
fn nth_dimension_is_value(toml: &toml::map::Map<std::string::String, toml::Value>, x: &str) -> bool {
    let mut list_of_indexes: Vec<&str> = x.split(".").collect::<Vec<&str>>();
    let value_index: &str = list_of_indexes.pop().unwrap();
    let tables_are_valid: bool = nth_dimension_is_table(toml, list_of_indexes.clone()); // check tables leading up to the value index exist
    if tables_are_valid {
        // Get bottom table by iterating through all of them. pain.
        let mut botmost_table: &toml::map::Map<std::string::String, toml::Value> = toml;
        for table in list_of_indexes {
            botmost_table = botmost_table[table].as_table().unwrap();
        }
        // check key exists // Then check key is not a table
        if botmost_table.contains_key(value_index) & !botmost_table[value_index].is_table() {
            return true
        }
        else {
            return false
        }
    }
    else {
        return false
    }
}

// Validate ulai file
fn sanity_ulai(path: PathBuf) -> bool {
    // Check that ulai path exists
    if PathBuf::from(&path).exists() {
        // Parse ulai file as one long string
        //let mut file = File::open(path).expect("Could not open ulai file");
        let ulai = read_to_string(path).expect("Could not read ulai file");
        // Parse the long string as a toml struct 
        let toml: toml::map::Map<std::string::String, toml::Value>  = parse_toml_struct_from_string(&ulai);
        // Check these pretty important values exist in the ulai (no distro specific stuff here, except maybe flatpak or snap checks)
        let has_pkg_name: bool = nth_dimension_is_value(&toml, "Metadata.pkg_name"); // Make sure packagename is included
        let has_distros: bool = nth_dimension_is_table_stringin(&toml, "Distros"); // Make sure there's a distro section at all
        let has_ulai_target: bool = nth_dimension_is_value(&toml, "Metadata.ulai_target"); // Make sure we have a targeted ulai version
        if has_pkg_name & has_distros & has_ulai_target {
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

// TODO: 
// - Create spec for .ulai files
// - Detect user distro and match to a set of package mangers
// - Handle dependencies
// - - 32 bit dependencies
// - Handle installation
// - Handle installation errors or complications
// - Handle updates