/// Provide wrappers for toplevel operations that require multiple
/// modules to work together.

use messages;
use config;
use benchmarking;
use statistics;

// Sender and Receiver live on the channel.
use std::sync::mpsc::channel;

// timepoints for time measurements
use std::time::Instant;

// save results in hashmap
use std::collections::BTreeMap;


/// Do all the benchmarks that are configured via the .yml file.
pub fn benchmarking_process(cfg_file: &str, threads: usize,
                            res_file: &str) -> i32 {
    // Read configuration for the benchmarks
    let bm_cfg = config::parse_config_file(cfg_file);

    // The channel is used to communicate the results of each benchmark.
    let (tx, rx) = channel();

    // start timer to measure overall runtime
    let start_all = Instant::now();

    // Schedule all wanted commands n times in a threadpool of n_workers
    // threads.
    let scheduled = benchmarking::schedule_benchmarks(bm_cfg, threads, tx);
    // Wait untill all scheduled commands are done and return the results.
    let (stats, successes, fails) = benchmarking::collect_results(scheduled, rx);

    // report the time and state of all benchmarks
    messages::report_runinformation(start_all.elapsed(), successes, fails);

    // report detailed benchmark statistics for each case
    report_data(&stats);
    messages::write_result_file(&res_file, &stats);

    0
}

/// Define the process of reporting the results of a benchmark.
pub fn reporting_process(result_file: &str) -> i32 {
    let bm_statistics = statistics::read_result_from_file(result_file);
    report_data(&bm_statistics)
}

/// Define the process of calculating and reporting the difference between
/// multiple benchmark runs.
pub fn diff_process(ground_truth: &str, results: &str, tolerance: f64) -> i32 {
    let gt_stats = statistics::read_result_from_file(ground_truth);
    let re_stats = statistics::read_result_from_file(results);

    messages::intro_diff(ground_truth, results);
    report_diff(&gt_stats, &re_stats, tolerance)
}


/// Report the results of a benchmark run.
fn report_data(times: &BTreeMap<String, Vec<f32>>) -> i32 {
    let stats = statistics::process_results(times);
    messages::report_statistics(&stats)
}

/// Report the difference between two benchmark results.
fn report_diff(ground_truth: &BTreeMap<String, Vec<f32>>,
               results: &BTreeMap<String, Vec<f32>>,
               tolerance: f64) -> i32 {
    let gt_stats = statistics::process_results(ground_truth);
    let re_stats = statistics::process_results(results);

    messages::report_diff(&gt_stats, &re_stats, tolerance)
}
