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


pub fn print_benchmarks(tui: &RustBox, bms: &Yaml) {
    /// print the headline
    tui.print(1,5, rustbox::RB_UNDERLINE, Color::White, Color::Default, "Runs");
    tui.print(8,5, rustbox::RB_UNDERLINE, Color::White, Color::Default, "Benchmark");

    let mut line = 7;
    for benchmark in bms["cases"].as_vec().unwrap() {
        tui.print(1,line, rustbox::RB_NORMAL, Color::Green, Color::Default, "1");
        
        let configured_name = benchmark["name"].as_str();
        match configured_name {
            Some(name) => tui.print(8,line, rustbox::RB_NORMAL, Color::White, Color::Default, name),
            None => tui.print(8,line, rustbox::RB_NORMAL, Color::White, Color::Default, "No Name provided"),
        }


        line = line + 1;
    }
}
