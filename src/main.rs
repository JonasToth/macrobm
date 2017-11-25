//! MacroBM is a tool to run macro benchmarks for arbitrary programs. It is easily configured via
//! .yml-Files and gives you some statistical information about the configured cases.
//!
//! The goal of this program is to provide an easy and extensible way to configure and run macro
//! benchmarks for your programs. Mainly developed to evaluate a numeric code, it wants to be
//! general :)

// command line parser
extern crate clap;
use clap::{Arg, App, SubCommand};

// colored output
extern crate term_painter;

// Parallel benchmarking
extern crate threadpool;

// yaml loading for configuration and result output
extern crate yaml_rust;

// link with statistics library
extern crate stat;

// Sender and Receiver live on the channel.
use std::sync::mpsc::channel;

// timepoints for time measurements
use std::time::Instant;

// save results in hashmap
use std::collections::BTreeMap;

// All messages that are reportable.
mod messages;
// parse the yaml configuration files and build the internal data structures
mod config;
// functions to do benchmarking
mod benchmarking;
// statistics for the durations
mod statistics;

fn report_data(times: &BTreeMap<String, Vec<f32>>) -> i32 {
    let stats = statistics::process_results(times);

    messages::report_statistics(&stats)
}

fn report_diff(ground_truth: &BTreeMap<String, Vec<f32>>,
               results: &BTreeMap<String, Vec<f32>>,
               tolerance: f64) -> i32 {
    let gt_stats = statistics::process_results(ground_truth);
    let re_stats = statistics::process_results(results);

    messages::report_diff(&gt_stats, &re_stats, tolerance)
}

fn main() {
    // ---------------- Configuration for the command line parser
    let matches = App::new("macrobm")
        .version("v0.4.1")
        .author("Jonas Toth <jonas.toth@gmail.com>")
        .about("Times execution time of commands and produces statistics.")
        .arg(Arg::with_name("config")
             .value_name("FILE")
             .help("Configuration for the macro benchmarks. Default: benchmarks.yml"))
        .arg(Arg::with_name("jobs")
             .short("j")
             .takes_value(true)
             .help("Control how many thread shall be used to run the benchmarks"))
        .arg(Arg::with_name("outfile")
             .short("o")
             .takes_value(true)
             .help("Set the filename for the raw data output file. Defaults to results.yml"))
        .subcommand(SubCommand::with_name("report")
                    .about("Print statistics of a previously run benchmark")
                    .arg(Arg::with_name("input")
                         .takes_value(true)
                         .help("Filename of the result file wanted to inspect. Defaults to results.yml")))
        .subcommand(SubCommand::with_name("diff")
                    .about("Compare two different result files with same benchmarks and show differences")
                    .arg(Arg::with_name("ground_truth")
                         .required(true)
                         .takes_value(true)
                         .help("Dataset we compare against."))
                    .arg(Arg::with_name("new_result")
                         .takes_value(true)
                         .help("Benchmark to compare against the ground truth. Defaults to results.yml"))
                    .arg(Arg::with_name("tolerance")
                         .short("t")
                         .takes_value(true)
                         .help("Modify tolerance in percent, to consider values as equal. Default is 2%")))
        .get_matches();

    // Handle subcommand for reporting.
    if let Some(sub_report) = matches.subcommand_matches("report") {
        let result_file = sub_report.value_of("input")
                                    .unwrap_or("results.yml");
        let bm_statistics = statistics::read_result_from_file(result_file);

        let return_code = report_data(&bm_statistics);

        std::process::exit(return_code);
    }
    // Compare different runs between each other
    else if let Some(sub_diff) = matches.subcommand_matches("diff") {
        let ground_truth_file = sub_diff.value_of("ground_truth").unwrap();
        let result_file = sub_diff.value_of("new_result")
                                  .unwrap_or("results.yml");

        let gt_stats = statistics::read_result_from_file(ground_truth_file);
        let re_stats = statistics::read_result_from_file(result_file);

        messages::intro_diff(ground_truth_file, result_file);

        let tolerance = sub_diff.value_of("tolerance").unwrap_or("2.")
                        .parse::<f64>().unwrap();
        let return_code = report_diff(&gt_stats, &re_stats, tolerance);

        std::process::exit(return_code);
    }
    // Default usage, run benchmarks.
    else {
        // ---------------- Read configuration for the benchmarks
        let cfg_file_name = matches.value_of("config")
            .unwrap_or("benchmarks.yml");
        let bm_cfg = config::parse_config_file(cfg_file_name);

        // --------------- Configure multithreading for the benchmarks
        let n_workers = matches.value_of("jobs").unwrap_or("1")
            .parse::<usize>().unwrap();

        let (tx, rx) = channel();

        // start timer to measure overall runtime
        let start_all = Instant::now();

        // Schedule all wanted commands n times in a threadpool of n_workers
        // threads.
        let scheduled = benchmarking::schedule_benchmarks(bm_cfg, n_workers, tx);
        // Wait untill all scheduled commands are done and return the results.
        let (stats, successes, fails) = benchmarking::collect_results(scheduled, rx);

        // report the time and state of all benchmarks
        messages::report_runinformation(start_all.elapsed(), successes, fails);

        // report detailed benchmark statistics for each case
        report_data(&stats);

        let result_file = matches.value_of("outfile").unwrap_or("results.yml");
        messages::write_result_file(&result_file, &stats);

        std::process::exit(0);
    }
}
