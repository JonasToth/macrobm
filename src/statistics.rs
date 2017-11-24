/// Calculate statistics from the measured durations

extern crate stat;
use stat::{mean, minmax, absdev};
use std::collections::BTreeMap;

use config::file_to_yaml;
use yaml_rust::{Yaml, YamlLoader};


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
pub fn process_results(run_statistic: &BTreeMap<String, Vec<f32>>)
    -> BTreeMap<String, BMStatistics> {
        let mut result = BTreeMap::new();

        for (bm_name, times) in run_statistic {
            assert!(times.len() > 0);
            let (min, _, max, _) = minmax(times);
            result.insert(bm_name.clone(),
            BMStatistics {
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
pub fn compare_runs(run1: &BTreeMap<String, BMStatistics>,
                    run2: &BTreeMap<String, BMStatistics>,
                    tolerance: f64)
    -> BTreeMap<String, ComparisonResult> {
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

            result.insert(bm_name.to_string(),
            ComparisonResult {
                avg: best_avg,
                min: best_min,
                max: best_max,
            });
        }
        result
    }

/// Compare two metrics for equality, tol(0. - 100.) is given in percent!
fn compare_single(value1: f64, value2: f64, tol: f64) -> Comparison {
    assert!(value1 != 0.);
    if (value1 - value2).abs() / value1.abs() <= (tol / 100.) {
        return Comparison::Equal;
    } else if value1 < value2 {
        return Comparison::OneIsFaster;
    } else {
        return Comparison::TwoIsFaster;
    }
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
    results_from_yaml(&yml)
}

fn results_from_yaml(doc: &Yaml) -> BTreeMap<String, Vec<f32>> {
    let mut result = BTreeMap::new();

    for single_result in doc.as_vec().unwrap() {
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


// ------------------------------- All tests for the statistic functionality -------------------

#[test]
fn test_process_results() {
    let mut collected_times = BTreeMap::new();
    collected_times.insert("simulation".to_string(), vec![15., 14., 16.]);
    let stats = process_results(&collected_times);
    let stats = stats.get("simulation").unwrap();

    assert_eq!(stats.avg, 15., "Avg is wrong");
    assert_eq!(stats.min, 14., "Min is wrong");
    assert_eq!(stats.max, 16., "Max is wrong");
    match compare_single(stats.dev, 0.6666, 0.5) {
        Comparison::Equal => (),
        _ => panic!("Dev is wrong"),
    };
    assert_eq!(stats.count, 3, "Count is wrong");
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_process_results_invalid() {
    let mut collected_times = BTreeMap::new();
    collected_times.insert("simulation".to_string(), Vec::<f32>::new());
    process_results(&collected_times);
}

#[test]
fn test_compare_runs() {
    // run 1 is faster then run 2
    let run1 = {
        let mut x = BTreeMap::<String, _>::new();
        x.insert("sleep".to_string(),
        BMStatistics {
            avg: 15.3,
            min: 15.1,
            max: 15.5,
            dev: 0.005,
            count: 100,
        });
        x.insert("not_in_other".to_string(),
        BMStatistics {
            avg: 1.,
            min: 0.9,
            max: 1.6,
            dev: 0.2,
            count: 3,
        });
        x
    };
    let run2 = {
        let mut x = BTreeMap::new();
        x.insert("sleep".to_string(),
        BMStatistics {
            avg: 16.3,
            min: 16.1,
            max: 16.5,
            dev: 0.005,
            count: 100,
        });
        x.insert("some_unused".to_string(),
        BMStatistics {
            avg: 1.,
            min: 0.9,
            max: 1.6,
            dev: 0.2,
            count: 3,
        });
        x
    };

    let cmp = compare_runs(&run1, &run2, 2.);
    // check outcome
    let c = cmp.get("sleep").unwrap();
    match c.avg {
        Comparison::OneIsFaster => (),
        _ => panic!("one is faster!"),
    }
    match c.min {
        Comparison::OneIsFaster => (),
        _ => panic!("one is faster!"),
    }
    match c.max {
        Comparison::OneIsFaster => (),
        _ => panic!("one is faster!"),
    }
    match cmp.get("not_in_other") {
        Some(_) => panic!("Not allowed in the result!"),
        None => (),
    }
    match cmp.get("some_unused") {
        Some(_) => panic!("Not allowed in the result!"),
        None => (),
    }

    // same as previous comparison, but swapped
    let cmp = compare_runs(&run2, &run1, 2.);
    // check outcome
    let c = cmp.get("sleep").unwrap();
    match c.avg {
        Comparison::TwoIsFaster => (),
        _ => panic!("two is faster!"),
    }
    match c.min {
        Comparison::TwoIsFaster => (),
        _ => panic!("two is faster!"),
    }
    match c.max {
        Comparison::TwoIsFaster => (),
        _ => panic!("two is faster!"),
    }
    match cmp.get("not_in_other") {
        Some(_) => panic!("Not allowed in the result!"),
        None => (),
    }
    match cmp.get("some_unused") {
        Some(_) => panic!("Not allowed in the result!"),
        None => (),
    }

    // same as previous comparison, but equality because high tolerance
    let cmp = compare_runs(&run2, &run1, 50.);
    // check outcome
    let c = cmp.get("sleep").unwrap();
    match c.avg {
        Comparison::Equal => (),
        _ => panic!("result considered equal!"),
    }
    match c.min {
        Comparison::Equal => (),
        _ => panic!("result considered equal!"),
    }
    match c.max {
        Comparison::Equal => (),
        _ => panic!("result considered equal!"),
    }
    match cmp.get("not_in_other") {
        Some(_) => panic!("Not allowed in the result!"),
        None => (),
    }
    match cmp.get("some_unused") {
        Some(_) => panic!("Not allowed in the result!"),
        None => (),
    }
}

#[test]
fn test_compare_single() {
    match compare_single(1., 1., 0.) {
        Comparison::Equal => (),
        _ => panic!("Expected equality!"),
    }

    match compare_single(1., 1., 0.01) {
        Comparison::Equal => (),
        _ => panic!("Expected equality!"),
    }

    match compare_single(100., 105., 1.) {
        Comparison::OneIsFaster => (),
        _ => panic!("Expected one is faster!"),
    }

    match compare_single(105., 100., 1.) {
        Comparison::TwoIsFaster => (),
        _ => panic!("Expected two is faster!"),
    }

    match compare_single(100., 103., 4.) {
        Comparison::Equal => (),
        _ => panic!("Expected equality!"),
    }
}

#[test]
fn test_relative_variance() {
    let mut ez_stats = BMStatistics {
        avg: 15.,
        min: 0.,
        max: 0.,
        dev: 0.,
        count: 100,
    };
    assert_eq!(calc_relative_variance(&ez_stats), 0.);

    ez_stats.avg = 100.;
    ez_stats.dev = 1.;
    assert_eq!(calc_relative_variance(&ez_stats), 1.);
}

#[test]
fn test_read_result() {
    let result_str = "---
    - program1:
      - 0.9
      - 1.1
      - 1.2
      - 1.3
      - 1.5";
    let yaml = YamlLoader::load_from_str(result_str).unwrap();

    let result = results_from_yaml(&yaml[0]);

    assert_eq!(result.get("program1").unwrap()[0], 0.9);
    assert_eq!(result.get("program1").unwrap()[1], 1.1);
    assert_eq!(result.get("program1").unwrap()[2], 1.2);
    assert_eq!(result.get("program1").unwrap()[3], 1.3);
    assert_eq!(result.get("program1").unwrap()[4], 1.5);
}
