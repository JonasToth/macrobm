/// We parse configs cause we are so nice

use yaml_rust::{Yaml,YamlLoader};

use bm_runconfig::RunConfig;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use messages;

/// Parse the config file and create internal data structure used to spawn a benchmark.
pub fn parse_config(file_name: &str) -> HashMap<String, RunConfig> {
    let yaml_doc = file_to_yaml(file_name);
    let doc = &yaml_doc[0];
    let mut cfg = HashMap::<String, RunConfig>::new();

    // default values, that can be set global for all cases
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
        let cmd_slice = bm["command"].as_str().unwrap();
        let key = bm["name"].as_str().unwrap_or(cmd_slice).to_string();
        let args = match bm["args"].as_vec() {
            Some(v) => yaml_args_to_stringlist(v),
            None    => default_args.clone(),
        };

        // fill configuration with values and/or default values
        let cfg_struct = RunConfig{
            name: key.clone(),
            description: bm["description"].as_str().unwrap_or("").to_string(),
            count: bm["count"].as_i64().unwrap_or(default_count),
            
            command: cmd_slice.to_string(),
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

/// Read in a file and try to generate yml out of it. Will panic if yaml cant be loaded.
pub fn file_to_yaml(file_name: &str) -> Vec<Yaml> {
    // open the file
    let mut config_file = match File::open(file_name) {
        Ok(file) => file,
        Err(e)   => { messages::invalid_config_filename(file_name);
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
