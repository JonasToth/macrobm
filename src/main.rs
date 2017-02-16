// this program is about "macro benchmarking", in the sense that it will measure whole programm
// execution and is able to compare run times.
// Provide shell scripts that shall be measured!

// error handling
//use std::error::Error;
//use std::default::Default;

// command line parser
extern crate clap;
use clap::{Arg, App};

extern crate term_painter;

// yaml loading for configuration and result output
extern crate yaml_rust;
use yaml_rust::YamlLoader;

// time measurement and stuff
use std::fs;
use std::io::Read;

// subprocesses to call the command we want to measure
use std::process::{Command, Stdio};

// time measurement and and threading
extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::time::Instant;

// custom functions written by me, for code clearity
mod messages;
// parse the yaml configuration files and build the internal data structures
mod config;


fn main() {
    /// ---------------- Configuration for the command line parser
    let matches = App::new("macrobm")
                       .version("v0.1-beta")
                       .author("Jonas Toth <jonas.toth@gmail.com>")
                       .about("Times execution time of commands and produces statistics.")
                       .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Configuration for the macro benchmarks")
                       )
                       .arg(Arg::with_name("jobs")
                                .short("j")
                                .takes_value(true)
                                .help("Control how many thread shall be used to run the benchmarks")
                        )
                       // TODO add option for execution directory
                       // TODO add option for output file .yml
                       .get_matches();

    /// ---------------- Read configuration for the benchmarks
    let cfg_file_name = matches.value_of("config").unwrap_or("benchmarks.yml");
    let mut config_file = match fs::File::open(cfg_file_name) {
        Ok(file) => file,
        Err(e)   => { messages::invalid_config_filename(cfg_file_name);
                      panic!("{}", e); }
    };
    let mut config_file_content = String::new();
    config_file.read_to_string(&mut config_file_content).unwrap();
    let bm = YamlLoader::load_from_str(&config_file_content).unwrap();

    /// --------------- Build the List of Commands necessary to start
    let bms = &bm[0];

    /// --------------- Present first view

    /// --------------- Configure multithreading for the benchmarks
    /// --------------- Setup communication structure for timing results
    let n_workers = matches.value_of("jobs").unwrap_or("1");
    let n_workers = n_workers.parse::<usize>().unwrap();
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    messages::intro(n_workers);

    let mut scheduled = 0;

    for benchmark in bms["cases"].as_vec().unwrap() {
        let runcount = benchmark["count"].as_i64().unwrap_or(1);
        let name_str = benchmark["name"].as_str().unwrap().to_string();

        messages::scheduled_command(name_str, runcount);

        for _ in 0..runcount {
            let tx = tx.clone();
            // so ugly, the parsing should be outside the loop, and copies should be made
            let command_str = benchmark["command"].as_str().unwrap().to_string();
            let args = benchmark["args"].as_vec().unwrap();
            let argument_list = config::yaml_args_to_stringlist(args);

            pool.execute(move || {
                //messages::start_program(name_str);

                let start_time = Instant::now();
                let mut child = Command::new(command_str)
                                        .args(&argument_list)
                                        .stdout(Stdio::null())
                                        .stderr(Stdio::null())
                                        .spawn()
                                        .expect("program failed");
                let _ = child.wait()
                                 .expect("failed to wait on programm");

                /// build execution report
                let execution_time = start_time.elapsed().as_secs();
                tx.send(execution_time).unwrap();
            });
            scheduled+= 1;
        }
    } 

    println!("");

    for _ in 0..scheduled {
        let report = rx.recv().unwrap();
        // process report
        
        // output information
        messages::finished_program(report);
    }

    // output final report / file output

    messages::finished();
}
