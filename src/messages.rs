extern crate term_painter;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use yaml_rust::{Yaml, YamlEmitter};

// progress bar in cmd line
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::BTreeMap;

use report::Report;
use statistics::BMStatistics;


/// Banner printed in every programm run.
pub fn intro(worker: usize) {
    println!("Running {} {} threads", Blue.bold().paint("macro benchmarks"), worker);
}

pub fn report_statistics(stats: &BTreeMap<String, BMStatistics>) {
    println!("{:6} {:10} {:8} {:7} {:7} {:20}", 
             Blue.bold().paint("Runs"), Blue.bold().paint("Avg"), 
             Blue.bold().paint("Dev"), Blue.bold().paint("Min"), Blue.bold().paint("Max"),
             Blue.bold().paint("Name"));

    for (bm_name, stat) in stats {
        let reldev = stat.dev / stat.avg;
        println!("{:6} {:8.2} {:3.1}% {:8.2} {:8.2} {}", stat.count, Bold.paint(stat.avg), 
                                                         reldev, stat.min, stat.max, 
                                                         Bold.paint(bm_name));
    }
}

/// Divider.
pub fn intro_report() {
    println!("{}", Bold.paint("==========================================================================="));
}

/// Error message for an invalid configuration file for benchmarks.
pub fn invalid_config_filename(fname: &str) {
    println!("{} could not open file {} as config.", Red.bold().paint("Failure"), Red.paint(fname));
}

pub fn invalid_yaml(fname: &str) {
    println!("Error while parsing yml file {}!", Red.paint(fname));
}

/// Gets called when a command gets scheduled count-times. Information for user.
pub fn scheduled_command(name: &str, count: i64) {
    println!("{} {} for {} runs", Blue.paint("Scheduling"), Bold.paint(name), 
                                  Bold.paint(count));
}

/// Gets called whenever one run of a benchmark is finished. Producess progressbar effect
pub fn finished_program(report: Report, counter: i64, maximum: i64)
{
    // err_code.success()
    let name = if report.ecode.success() { Green.bold().paint(report.name) } 
                       else { Red.bold().paint(report.name) };
    let exec_time = report.duration;

    clean_line();
    print!("\r{} took {:.2} {} {}{}{}{}{}",
           Bold.paint(name), Bold.paint(exec_time),
           Bold.paint("seconds"), Blue.paint("["), Blue.paint(counter), Blue.paint("/"),
           Blue.paint(maximum), Blue.paint("]"));

    io::stdout().flush().ok().expect("Could not flush stdout");
}

/// Gets called when all benchmarks were run.
pub fn finished() {
    clean_line();
    println!("\r{}", Blue.bold().paint("Finished running benchmarks.!"));
}

/// Write the measured times as Yaml to the specified file. Casename is the key, value is a vector
/// of floats.
pub fn write_result_file(filename: &str, results: &BTreeMap<String, Vec<f32>>) {
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
            Ok(_) => (),
            Err(e) => {println!("Error while creating the result file!"); panic!(e);},
        }
    }

    let mut file = File::create(filename).expect("Could not open result file");
    file.write_all(out_str.as_bytes()).expect("Could not write results");
}

/// Clean the current line. Used for the progressbar effect.
fn clean_line() {
    print!("\r                                                                                                                   ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}
