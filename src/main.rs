// this program is about "macro benchmarking", in the sense that it will measure whole programm
// execution and is able to compare run times.
// Provide shell scripts that shall be measured!

// error handling
use std::error::Error;
use std::default::Default;

// command line parser
extern crate clap;
use clap::{Arg, App};

// yaml loading for configuration and result output
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

// terminal user interface
extern crate term_painter;
use term_painter::*;

//extern crate rustbox; 
//use rustbox::{Color, RustBox};
//use rustbox::Key;

// time measurement and stuff
use std::{thread, time, fs};
use std::io::Read;

// subprocesses to call the command we want to measure
use std::process::Command;

// time measurement and and threading
extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

// custom functions written by me, for code clearity
mod messages;


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
                       .get_matches();

    /// ---------------- Read configuration for the benchmarks
    let mut config_file = fs::File::open("macro.yml").unwrap();
    let mut config_file_content = String::new();
    config_file.read_to_string(&mut config_file_content).unwrap();
    let bm = YamlLoader::load_from_str(&config_file_content).unwrap();

    /// --------------- Build the List of Commands necessary to start
    let bms = &bm[0];

    /// --------------- Present first view
    messages::intro();

    /// --------------- Configure multithreading for the benchmarks
    /// --------------- Setup communication structure for timing results
    let n_workers = matches.value_of("jobs").unwrap_or("1");
    let n_workers = n_workers.parse::<usize>().unwrap();
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    let mut scheduled = 0;

    for benchmark in bms["cases"].as_vec().unwrap() {
        println!("Count");
        let runcount = benchmark["count"].as_i64().unwrap();
        println!("{}", runcount);

        for _ in 0..runcount {
            let tx = tx.clone();
            // so ugly, the parsing should be outside the loop, and copies should be made
            println!("Name");
            let name_str = benchmark["name"].as_str().unwrap().to_string();
            println!("Command");
            let command_str = benchmark["command"].as_str().unwrap().to_string();
            println!("Args");
            let args = benchmark["args"].as_vec().unwrap();
            let argument_list = yaml_args_to_stringlist(args);
            println!("{:?}", args);
            println!("Thread");
            pool.execute(move || {
                messages::start_program(name_str);

                let start_time = Instant::now();
                let mut child = Command::new(command_str)
                                        //.args(args)
                                        .arg("1")
                                        .spawn()
                                        .expect("program failed");
                let ecode = child.wait()
                                 .expect("failed to wait on programm");

                /// build execution report
                let execution_time = start_time.elapsed().as_secs();
                tx.send(execution_time).unwrap();

                messages::finished_program(execution_time);
            });
            scheduled+= 1;
        }
    } 
    for _ in 0..scheduled {
        let report = rx.recv().unwrap();
        println!("Reported {}", report);
    }

    messages::finished();
}
