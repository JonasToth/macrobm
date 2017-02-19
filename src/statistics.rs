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

pub struct ComparisonResult {
    pub avg: Comparison,
    pub min: Comparison,
    pub max: Comparison,
}

/// Compare two benchmark runs against each other. This holds the relevant information who won.
pub enum Comparison {
    Equal,
    OneIsFaster,
    TwoIsFaster,
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

/// Compare two runs of the same benchmark against each other and store which one one (with the
/// given percantage of tolerance for equality).
pub fn compare_runs(run1: &BTreeMap<String, BMStatistics>, run2: &BTreeMap<String, BMStatistics>, tolerance: f64) -> BTreeMap<String, ComparisonResult> {
    let mut result = BTreeMap::new();
    
    for bm_name in run1.keys() {
        let stat1 = run1.get(bm_name).unwrap();
        let stat2 = match run2.get(bm_name) {
            Some(stats) => stats,
            None => continue,
        };
        let best_min = compare_single(stat1.min, stat2.min, tolerance);
        let best_max = compare_single(stat1.max, stat2.max, tolerance);
        let best_avg = compare_single(stat1.avg, stat2.avg, tolerance);

        result.insert(bm_name.to_string(), ComparisonResult {
            avg: best_avg,
            min: best_min,
            max: best_max,
        });
    }

    result
}

/// Compare two metrics for equality, tol(0. - 100.) is given in percent!
fn compare_single(value1: f64, value2: f64, tol: f64) -> Comparison {
    if (value1 - value2).abs() / value1.abs() < (tol / 100.) { return Comparison::Equal; }
    else if value1 < value2 { return Comparison::OneIsFaster; }
    else { return Comparison::TwoIsFaster; }
}

/// Calculate the procentual variance for that case. 100. * stddev / avg
pub fn calc_relative_variance(statistics: &BMStatistics) -> f64 {
    100. * statistics.dev / statistics.avg
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
