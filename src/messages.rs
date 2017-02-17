extern crate term_painter;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use yaml_rust::{Yaml, YamlEmitter};

// progress bar in cmd line
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BTreeMap;

use execution_report::Report;


pub fn intro(worker: usize) {
    println!("Running {} {} threads", Blue.bold().paint("macro benchmarks"), worker);
}

pub fn intro_report() {
    println!("{}", Bold.paint("==========================================================================="));
}

pub fn invalid_config_filename(fname: &str) {
    println!("{} could not open file {} as config.", Red.bold().paint("Failure"), Red.paint(fname));
}

pub fn scheduled_command(name: &str, count: i64) {
    println!("{} {} for {} runs", Blue.paint("Scheduling"), Bold.paint(name), 
                                  Bold.paint(count));
}

pub fn finished_program(report: Report, counter: i32, maximum: i32)
{
    // err_code.success()
    let state_string = if report.ecode.success() { Green.bold().paint("Success") } 
                       else { Red.bold().paint("Failure") };
    let exec_time = report.duration;

    clean_line();
    print!("\r{} {} after {:.2} {} {}{}{}{}{}",
           state_string, Bold.paint(report.name), Bold.paint(exec_time),
           Bold.paint("seconds"), Blue.paint("["), Blue.paint(counter), Blue.paint("/"),
           Blue.paint(maximum), Blue.paint("]"));

    io::stdout().flush().ok().expect("Could not flush stdout");
}

pub fn finished() {
    clean_line();
    println!("\r{}", Blue.bold().paint("Finished running benchmarks.!"));
}

pub fn write_result_file(filename: &str, results: &HashMap<String, Vec<f32>>) {
    let mut case_vec = Vec::new();

    for case in results.keys() {
        // convert f32 times into yaml real values (strings)
        let mut yaml_times = Vec::new();
        for time in results.get(case).unwrap() {
            yaml_times.push(Yaml::Real(time.to_string()));
        }

        let mut hash_table = BTreeMap::new();
        hash_table.insert(Yaml::String(case.clone()), Yaml::Array(yaml_times));
        // push back the values to the case name
        case_vec.push(Yaml::Hash(hash_table));
    }

    let yaml_data = Yaml::Array(case_vec);
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        match emitter.dump(&yaml_data) {
            Ok(t) => (),
            Err(e) => {println!("Error while creating the result file!"); panic!(e);},
        }
    }

    let mut file = File::create(filename).expect("Could not open result file");
    file.write_all(out_str.as_bytes()).expect("Could not write results");
}

fn clean_line() {
    print!("\r                                                                                                                   ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}
