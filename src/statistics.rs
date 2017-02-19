/// Calculate statistics from the measured durations

extern crate stat;
use stat::{mean, minmax, absdev};
use std::collections::{BTreeMap};

use config::file_to_yaml;


#[derive(Debug)]
pub struct BMStatistics {
    /// average runtime of the benchmark
    pub avg: f64,
    /// shortest runtime of all runs
    pub min: f64,
    /// longest runtime of all runs
    pub max: f64,
    /// std deviation of the results, asuming a normal distribution of the runtimes
    pub dev: f64,
    /// number of measurements
    pub count: usize,
}



/// Postprocess the results of all benchmark runs. Currently only prints a table with most
/// interesting information.
pub fn process_results(run_statistic: &BTreeMap<String, Vec<f32>>) -> BTreeMap<String, BMStatistics> {
    let mut result = BTreeMap::new();

    for (bm_name, times) in run_statistic {
        let (min, _, max, _) = minmax(times);
        result.insert(bm_name.clone(),
            BMStatistics{
                avg: mean(times),
                min: min,
                max: max,
                dev: absdev(times),
                count: times.len(),
            });
    }
    result
}

/// Read in a result file and return all execution times mapped to their command name.
/// Panics if the file is not existing or the yaml cant be loaded.
pub fn read_result_from_file(file_name: &str) -> BTreeMap<String, Vec<f32>> {
    let yml = file_to_yaml(file_name);
    let yml = &yml[0];
    let mut result = BTreeMap::new();

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
