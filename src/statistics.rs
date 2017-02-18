/// Calculate statistics from the measured durations

extern crate stat;
use stat::{mean, minmax, absdev};
use std::collections::{HashMap};
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use config::file_to_yaml;

/// Postprocess the results of all benchmark runs. Currently only prints a table with most
/// interesting information.
pub fn process_results(run_statistic: &HashMap<String, Vec<f32>>) {

    println!("{:10} {:8} {:7} {:7} {:20}", Blue.bold().paint("Avg"), Blue.bold().paint("Dev"),
                               Blue.bold().paint("Min"), Blue.bold().paint("Max"),
                               Blue.bold().paint("Name"));

    for bm_name in run_statistic.keys() {
        let ref times = run_statistic.get(bm_name).unwrap();
        let avg = mean(times);
        let (min, _, max, _) = minmax(times);
        let dev = absdev(times);
        let reldev = dev / avg * 100.;

        println!("{:8.2} {:3.1}% {:8.2} {:8.2} {}", Bold.paint(avg), reldev, min, 
                                                    max, Bold.paint(bm_name));
    }
}

/// Read in a result file and return all execution times mapped to their command name.
/// Panics if the file is not existing or the yaml cant be loaded.
pub fn read_result_from_file(file_name: &str) -> HashMap<String, Vec<f32>> {
    let yml = file_to_yaml(file_name);
    let yml = &yml[0];
    let mut result = HashMap::new();

    for single_result in yml.as_vec().unwrap() {
        let single_result = single_result.as_hash().unwrap();

        for (name, times) in single_result {
            let mut times_float = Vec::new();

            for el in times.as_vec().unwrap() {
                times_float.push(el.as_f64().unwrap() as f32);
            }
            result.insert(name.as_str().unwrap().to_string(), times_float);
        }
    }

    result
}
