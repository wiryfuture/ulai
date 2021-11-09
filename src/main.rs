use std::path::PathBuf;
use structopt::StructOpt;
use toml::{Value};
use toml::map::Map;
use std::fs::read_to_string;

//  Arguments for ulai
#[derive(StructOpt, Clone)]
struct Opt {
    #[structopt(short, long, parse(from_os_str), help="path of the ulai file for the app that will be installed")]
    // Directory of ulai file to use
    ulai: PathBuf,
    #[structopt(subcommand)]
    // Ulai commands
    cmd: Commands,
}
//  Subcommands for ulai
#[derive(StructOpt, Clone, PartialEq)]
#[structopt(name = "ulai", about = "distro-universal linux app installer")]
enum Commands {
    #[structopt(name = "validate", about = "validate if a ulai config can be parsed")]
    Validate,

    #[structopt(name = "install", about="install an app using a ulai config")]
    Install,
}

//  I don't understand what this does after the value parse so it go into its own function 
//  https://stackoverflow.com/a/57947338
//  what??
fn parse_toml_struct_from_string(file: &String) -> toml::map::Map<std::string::String, toml::Value> {
    file.parse::<Value>().ok().and_then(|r| match r {
        Value::Table(table) => Some(table),
        _ => None
    }).unwrap_or(Map::new())
}

//  Check that this table (or sets of tables) exist within the toml struct
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
//  Wrapper for function above that splits the indexes passed as a vector for you
fn nth_dimension_is_table_stringin(toml: &toml::map::Map<std::string::String, toml::Value>, table_indexes: &str) -> bool {
    nth_dimension_is_table(toml, table_indexes.split(".").collect::<Vec<&str>>())
}
// Actually get value of nth table  //  little to no checking, above functions should be ran first. 
fn nth_dimension_get_table(toml: &toml::map::Map<std::string::String, toml::Value>, table_indexes: &str) -> toml::map::Map<std::string::String, toml::Value> {
    let indexes: Vec<&str> = table_indexes.split(".").collect::<Vec<&str>>(); // This isn't working
    let mut current_table_layer = toml;
    println!("Current layer: {:?}", current_table_layer);
    for current_index in indexes {
        current_table_layer = current_table_layer[current_index].as_table().unwrap();
        //println!("There is a table called {} in this toml", element);
    }
    return current_table_layer.clone(); 
}

//  Takes a set of indexes and confirms whether a value is there (NOT A TABLE!) by iterating through each table and then the value itself
fn nth_dimension_is_value(toml: &toml::map::Map<std::string::String, toml::Value>, indexes: &str) -> bool {
    let mut list_of_indexes: Vec<&str> = indexes.split(".").collect::<Vec<&str>>();
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
// Actual get a value from an index //  Little to no checking, above should have been ran first
fn nth_dimension_get_value(toml: &toml::map::Map<std::string::String, toml::Value>, indexes: &str) -> String {
    let mut list_of_indexes: Vec<&str> = indexes.split(".").collect::<Vec<&str>>();
    let value_index: &str = list_of_indexes.pop().unwrap();
    // Get parent table of the value
    let parent_table: toml::map::Map<std::string::String, toml::Value> = nth_dimension_get_table(toml, &list_of_indexes.concat().to_string());
    // Get value from its parent table (hopefully)
    parent_table[value_index].to_string()
}

//  Check that the inheritance of a distro resolves (to something)
/*
    Inheritance should resolve at any layer of nesting
    Example:
        [DISTROS.Fedora.35]
        inherit="Fedora.34"

        [DISTROS.Fedora.34]
        inherit="Fedora.33"

        [Distros.Fedora.33]
        repo-"https://example.com"
    Fedora.35 AND Fedora.34 should BOTH resolve to Fedora.33, even though Fedora.35 is inheriting Fedora.34
*/
//  At this point, the existence of the first "inherit" value we're using should be undisputed. It should be laid out as "Distro.Ver".
//  Optionally, "Distro.Ver.Deps" and "Distro.Ver.Repos" should both be able to be inherited separately as well (advanced inheritance)
fn nth_table_inheritance(toml: &toml::map::Map<std::string::String, toml::Value>, indexes: &str) -> bool {
    //  need to get our toml to the distro table level first
    //  Gonna assume the distro table exists at this point
    let distro_toml: &toml::map::Map<std::string::String, toml::Value> = toml["Distros"].as_table().unwrap();
    let mut current_indexes = indexes.to_string();
    let value_exists = loop {
        //  indexes_exist should be bool for whether the inherit values resolve to a parent
        let indexes_exist: bool = nth_dimension_is_table_stringin(&distro_toml, &indexes);
        println!("Does table {} exist?: {}", indexes, indexes_exist);
        //  Check whether the parent has an inherit key
        if indexes_exist {
            current_indexes += ".inherit";
            let parent_inherits_too: bool = nth_dimension_is_value(&distro_toml, indexes);
            // parent also has an inherit value
            if parent_inherits_too {
                // Set "indexes" to that of the parents' inherit value
                //indexes = distro_toml[indexes]
            }
        }
        //  Did not resolve to a parent, so this is false
        else {
            break false;
        }
    };
    return value_exists;
}
// Checks inheritance resolving but for a value
fn nth_value_inheritance(toml: &toml::map::Map<std::string::String, toml::Value>, indexes: &str) -> bool {

    false
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
            println!("Value of fedora 33 inherit: {}", nth_dimension_get_value(&toml, "Distros.Fedora.33.inherit"));
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
fn argparse(args: Opt) -> bool {
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
    let args = Opt::from_args();
    let arg_sanity = argparse(args.clone());
    
    // End program if running validate command
    if args.cmd == Commands::Validate {
        println!("Sanity check passed: {}", arg_sanity);
        return
    }
}
