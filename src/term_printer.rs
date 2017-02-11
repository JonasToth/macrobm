extern crate rustbox; 
use rustbox::{Color, RustBox};
use rustbox::Key;

extern crate yaml_rust;
use yaml_rust::{Yaml};


pub fn print_control_message(tui: &RustBox) {
    tui.print(1,1, rustbox::RB_BOLD, Color::White, Color::Default,
    "Running macro benchmarks");
    tui.print(1,2, rustbox::RB_BOLD, Color::White, Color::Default,
    "--------------------------------------------------------");
}


pub fn print_benchmarks(tui: &RustBox, bms: &Vec<Yaml>) {
    /// print the headline
    tui.print(1,5, rustbox::RB_UNDERLINE, Color::White, Color::Default, "Runs");
    tui.print(8,5, rustbox::RB_UNDERLINE, Color::White, Color::Default, "Benchmark");

    let mut line = 7;
    for benchmark in bms {
        tui.print(1,line, rustbox::RB_NORMAL, Color::Green, Color::Default, "1");
        tui.print(8,line, rustbox::RB_NORMAL, Color::White, Color::Default, "lol");
                  //benchmark["command"].as_str().unwrap());

        line = line + 1;
    }
}
