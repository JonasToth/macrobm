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
extern crate rustbox; 
use rustbox::{Color, RustBox};
use rustbox::Key;

// subprocesses
use std::process::Command;

// time measurement and stuff
use std::{thread, time, fs};
use std::io::Read;

// custom functions written by me, for code clearity
mod term_printer;


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
                       .get_matches();

    /// ---------------- Read configuration for the benchmarks
    let mut config_file = fs::File::open("macro.yml").unwrap();
    let mut config_file_content = String::new();
    config_file.read_to_string(&mut config_file_content).unwrap();
    let bm = YamlLoader::load_from_str(&config_file_content).unwrap();

    //println!("{:?}", bm);
    //println!("{:?}", bm[0]);

    /// ---------------- Configuration for the rustbox tui
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    /// ---------------- Present first view
    term_printer::print_control_message(&rustbox);
    rustbox.present();

    thread::sleep(time::Duration::from_millis(1000));

    loop {
        /// -------------- printing the status of the benchmarking
        rustbox.clear();
        term_printer::print_control_message(&rustbox);
        term_printer::print_benchmarks(&rustbox, &bm[0]);
        rustbox.present();

        /// --------------- key polling - control
        match rustbox.poll_event(false) {
            /// key input
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    Key::Char('p') => { 
                        rustbox.print(1, 4, rustbox::RB_NORMAL, Color::White, 
                                      Color::Default, "Someoutput"); 
                        rustbox.present();
                    }
                    _ => {}
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => {}
        }

        /// sleep for some milliseconds to reduce unnecessary cpu load
        thread::sleep(time::Duration::from_millis(50));
    }
}
