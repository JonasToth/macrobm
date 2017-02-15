extern crate term_painter;
use term_painter::ToStyle;
use term_painter::Attr::*;

extern crate yaml_rust;
use yaml_rust::{Yaml};


pub fn intro() {
    println!("{}", Bold.paint("Running macro benchmarks"));
    println!("");
}

pub fn start_program(name: String) {
    println!("Starting {}", Bold.paint(name));
}

pub fn finished_program(execution_time_seconds: u64)
{
    println!("Finished {} after {} seconds", Bold.paint("DummyRightNow"), Bold.paint(execution_time_seconds));
}

pub fn finished() {
    println!("Finished running benchmarks.!");
}


pub fn print_benchmarks(bms: &Yaml) {
    /*let mut line = 7;
    for benchmark in bms["cases"].as_vec().unwrap() {
        tui.print(1,line, rustbox::RB_NORMAL, Color::Green, Color::Default, "1");
        
        let configured_name = benchmark["name"].as_str();
        match configured_name {
            Some(name) => tui.print(8,line, rustbox::RB_NORMAL, Color::White, Color::Default, name),
            None => tui.print(8,line, rustbox::RB_NORMAL, Color::White, Color::Default, "No Name provided"),
        }


        line = line + 1;
    }
    */
}
