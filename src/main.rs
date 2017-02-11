// this program is about "macro benchmarking", in the sense that it will measure whole programm
// execution and is able to compare run times.
// Provide shell scripts that shall be measured!

// command line parser
extern crate clap;
use clap::{Arg, App};

// terminal user interface
extern crate rustbox; 
use rustbox::{Color, RustBox};
use rustbox::Key;

// error handling
use std::error::Error;
use std::default::Default;

// subprocesses
use std::process::Command;

// time measurement and stuff
use std::{thread, time};

// custom functions written by me, for code clearity
mod term_printer;
use term_printer::*;


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

    /// ---------------- Configuration for the rustbox tui
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    /// ---------------- Present first view
    term_printer::print_control_message(&rustbox);
    rustbox.present();

    loop {
        /// -------------- printing the status of the benchmarking
        rustbox.clear();
        term_printer::print_control_message(&rustbox);
        rustbox.print(1, 3, rustbox::RB_NORMAL, Color::White, Color::Default, ""); 

        rustbox.present();

        /// --------------- key polling - control
        match rustbox.poll_event(false) {
            /// key input
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }
                    Key::Char('p') => { 
                        rustbox.print(1, 4, rustbox::RB_NORMAL, Color::White, 
                                      Color::Default, "Penis"); 
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
