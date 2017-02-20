/// We parse configs cause we are so nice

use yaml_rust::{Yaml,YamlLoader};

use benchmarking::RunConfig;
use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;
use messages;

/// Read in a file and try to generate yml out of it. Will panic if yaml cant be loaded.
pub fn file_to_yaml(file_name: &str) -> Vec<Yaml> {
    // open the file
    let mut config_file = match File::open(file_name) {
        Ok(file) => file,
        Err(e)   => { messages::invalid_filename(file_name);
                      panic!("{}", e); }
    };

    let mut config_file_content = String::new();
    config_file.read_to_string(&mut config_file_content).unwrap();
    let yaml = match YamlLoader::load_from_str(&config_file_content) {
        Ok(vec)   => vec,
        Err(e)    => { messages::invalid_yaml(file_name);
                       panic!(e); },
    };

    yaml
}


/// Parse the config file and create internal data structure used to spawn a benchmark.
pub fn parse_config_file(file_name: &str) -> BTreeMap<String, RunConfig> {
    let yaml_doc = file_to_yaml(file_name);
    let doc = &yaml_doc[0];
    config_from_yaml(doc)
}

fn config_from_yaml(doc: &Yaml) -> BTreeMap<String, RunConfig> {
    let mut cfg = BTreeMap::<String, RunConfig>::new();

    // default values, that can be set global for all cases
    let default_cmd   = doc["command"].as_str().unwrap_or("");
    let default_count = doc["count"].as_i64().unwrap_or(1);
    let default_dir   = doc["directory"].as_str().unwrap_or(".");
    let default_args  = match doc["args"].as_vec() {
        Some(v) => yaml_args_to_stringlist(v),
        None    => Vec::<String>::new(),
    };
    let default_env   = match doc["environment"].as_vec() {
        Some(v) => yaml_stringarray_to_native(v),
        None    => vec!["".to_string()],
    };

    
    for bm in doc["cases"].as_vec().unwrap() {
        let cmd = bm["command"].as_str().unwrap_or(default_cmd).to_string();

        if cmd.is_empty() { panic!("No command provided for this benchmark!") }

        let key = bm["name"].as_str().unwrap_or(&cmd).to_string();
        let args = match bm["args"].as_vec() {
            Some(v) => yaml_args_to_stringlist(v),
            None    => default_args.clone(),
        };

        // fill configuration with values and/or default values
        let cfg_struct = RunConfig{
            name: key.clone(),
            description: bm["description"].as_str().unwrap_or("").to_string(),
            count: bm["count"].as_i64().unwrap_or(default_count),
            
            command: cmd,
            args: args,
            directory: bm["directory"].as_str().unwrap_or(default_dir).to_string(),
            environment: match bm["environment"].as_vec() {
                Some(v) => yaml_stringarray_to_native(v),
                None    => default_env.clone(),
            }
        };

        cfg.insert(key, cfg_struct);
    }

    cfg
}

/// Parse a yaml-Vector to strings, to use this list of strings as argument to start the wanted
/// process.
fn yaml_args_to_stringlist(args: &Vec<Yaml>) -> Vec<String>
{
    let mut result: Vec<String> = Vec::new();

    for arg_candidate in args {
        match arg_candidate {
            &Yaml::Real(ref a)   => result.push(a.clone()),
            &Yaml::Integer(ref a)=> result.push(a.to_string()),
            &Yaml::String(ref a) => result.push(a.clone()),
            &Yaml::Null          => (),
            _                    => panic!("Wrong argument type passed for that command!"),
        }
    }

    result
}

/// The same as yaml_args_to_stringlist, but accepts only Yaml::String.
fn yaml_stringarray_to_native(strings: &Vec<Yaml>) -> Vec<String> {
    let mut result = Vec::<String>::new();

    for s in strings {
        match s {
            &Yaml::String(ref str) => result.push(str.clone()),
            _                      => panic!("Expected a yaml-String. Provide correct data!"),
        }
    }
    result
}



// --------------------- Test for parsing the config files ------------------

#[test]
fn test_yaml_args_to_strings() {
    let yaml_args = vec![Yaml::Real("0.234".to_string()), Yaml::Integer(15), 
                         Yaml::String("hallo".to_string()), Yaml::Null];
    let strings = yaml_args_to_stringlist(&yaml_args);
    
    assert_eq!(strings.len(), 3);
    assert_eq!(strings[0], "0.234");
    assert_eq!(strings[1], "15");
    assert_eq!(strings[2], "hallo");
}

#[test]
#[should_panic]
fn test_yaml_args_to_strings_failcondition() {
    let yaml_args = vec![Yaml::Array(vec![Yaml::Integer(1),Yaml::Integer(2),
                                          Yaml::Integer(3)])];
    yaml_args_to_stringlist(&yaml_args);
}


#[test]
fn test_yaml_strings_to_native_strings() {
    let yaml_string = vec![Yaml::String("Hallo".to_string()), 
                           Yaml::String("Welt".to_string())];
    let native_string = yaml_stringarray_to_native(&yaml_string);
    
    assert_eq!(native_string[0], "Hallo");
    assert_eq!(native_string[1], "Welt");
}

#[test]
#[should_panic]
fn test_yaml_strings_to_native_strings_failcondition() {
    let yaml_string = vec![Yaml::Integer(1), Yaml::Real("0.5123".to_string())];
    yaml_stringarray_to_native(&yaml_string);
}







