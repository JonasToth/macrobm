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

use std::time::Duration;

use benchmarking::Report;
use statistics;
use statistics::{BMStatistics, Comparison};


/// Banner printed in every programm run.
pub fn intro(worker: usize) {
    println!("Running {} {} threads",
             Blue.bold().paint("macro benchmarks"),
             worker);
}

/// Output run statistics either collected or read in from a result file.
pub fn report_statistics(stats: &BTreeMap<String, BMStatistics>) -> i32 {
    println!("{:^6} {:^10} {:^10} {:^7} {:^10} {:<20}",
             Blue.bold().paint("Runs"),
             Blue.bold().paint("Min"),
             Blue.bold().paint("Avg"),
             Blue.bold().paint("Dev"),
             Blue.bold().paint("Max"),
             Blue.bold().paint("Name"));

    for (bm_name, stat) in stats {
        let reldev = 100. * stat.dev / stat.avg;
        println!("{:^6} {:^10.2} {:^10.2} +-{:^4.1}% {:^10.2} {:<20}",
                 stat.count,
                 stat.min,
                 Bold.paint(stat.avg),
                 reldev,
                 stat.max,
                 Bold.paint(bm_name));
    }

    return 0;
}

/// Print out how two runs differ. With nice coloring where changes are.
pub fn report_diff(gt_stats: &BTreeMap<String, BMStatistics>,
                   result_stat: &BTreeMap<String, BMStatistics>,
                   tolerance: f64) -> i32 {
    let comparison = statistics::compare_runs(gt_stats, result_stat, tolerance);

    for (bm_name, cmp) in comparison {
        // gt = Ground Truth
        // re = Result
        let gt = gt_stats.get(&bm_name).unwrap();
        let re = match result_stat.get(&bm_name) {
            Some(stats) => stats,
            None => continue,
        };

        // color the output depending which of the metric is better for which data set
        let (gt_min, re_min) = match cmp.min {
            Comparison::OneIsFaster => (Green.paint(gt.min), Red.paint(re.min)),
            Comparison::TwoIsFaster => (Red.paint(gt.min), Green.paint(re.min)),
            Comparison::Equal => (Plain.paint(gt.min), Plain.paint(re.min)),
        };

        let (gt_max, re_max) = match cmp.max {
            Comparison::OneIsFaster => (Green.paint(gt.max), Red.paint(re.max)),
            Comparison::TwoIsFaster => (Red.paint(gt.max), Green.paint(re.max)),
            Comparison::Equal => (Plain.paint(gt.max), Plain.paint(re.max)),
        };

        let (gt_avg, re_avg) = match cmp.avg {
            Comparison::OneIsFaster => (Green.bold().paint(gt.avg), Red.bold().paint(re.avg)),
            Comparison::TwoIsFaster => (Red.bold().paint(gt.avg), Green.bold().paint(re.avg)),
            Comparison::Equal => (Bold.paint(gt.avg), Bold.paint(re.avg)),
        };

        let reldev = statistics::calc_relative_variance(gt);
        print!("{:^6} {:^10.2} {:^10.2} +-{:^4.1}% {:^10.2} {:^20} ",
               gt.count,
               gt_min,
               gt_max,
               reldev,
               gt_avg,
               Bold.paint(bm_name));
        let reldev = statistics::calc_relative_variance(re);
        print!("{:^10.2} +-{:^4.1}% {:^10.2} {:^10.2} {:^6}",
               re_avg,
               reldev,
               re_min,
               re_max,
               re.count);
        println!("");
    }

    return 0;
}

pub fn intro_diff(gt_filename: &str, res_filename: &str) {
    print!("{:^47}", Blue.bold().paint(gt_filename));
    print!("{:22}", Blue.bold().paint("====================="));
    println!("{:^47}", Blue.bold().paint(res_filename));

    println!("{:^6} {:^10} {:^10} {:^7} {:^10} {:^20} {:^10} {:^7} {:^10} {:^10} {:^6}",
             Blue.bold().paint("Runs"),
             Blue.bold().paint("Min"),
             Blue.bold().paint("Max"),
             Blue.bold().paint("Dev"),
             Blue.bold().paint("Avg"),
             Blue.bold().paint("Name"),
             Blue.bold().paint("Avg"),
             Blue.bold().paint("Dev"),
             Blue.bold().paint("Min"),
             Blue.bold().paint("Max"),
             Blue.bold().paint("Runs"));
}

/// Error message for an invalid configuration file for benchmarks.
pub fn invalid_filename(fname: &str) {
    println!("{} could not open file {} for processing.",
             Red.bold().paint("Failure"),
             Red.paint(fname));
}

/// Error message when invalid yml was in a file.
pub fn invalid_yaml(fname: &str) {
    println!("Error while parsing yml file {}!", Red.paint(fname));
}

/// Gets called when a command gets scheduled count-times. Information for user.
pub fn scheduled_command(name: &str, count: i64) {
    println!("{} {} for {} runs",
             Blue.paint("Scheduling"),
             Bold.paint(name),
             Bold.paint(count));
}

/// Gets called whenever one run of a benchmark is finished. Producess progressbar effect
pub fn finished_program(report: &Report, counter: i64, maximum: i64) {
    // err_code.success()
    let name = if report.ecode.success() {
        Green.bold().paint(&report.name)
    } else {
        Red.bold().paint(&report.name)
    };
    let exec_time = report.duration;

    clean_line();
    print!("\r{} took {:.2} {} {}{}{}{}{}",
           Bold.paint(name),
           Bold.paint(exec_time),
           Bold.paint("seconds"),
           Blue.paint("["),
           Blue.paint(counter),
           Blue.paint("/"),
           Blue.paint(maximum),
           Blue.paint("]"));

    io::stdout().flush().ok().expect("Could not flush stdout");
}

pub fn report_runinformation(time: Duration, success_count: i32, fail_count: i32) {
    println!("All benchmarks took {} seconds", Bold.paint(time.as_secs()));
    println!("{} commands failed", Red.bold().paint(fail_count));
    println!("{} commands succeeded", Green.bold().paint(success_count));
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
            Err(e) => {
                println!("Error while creating the result file!");
                panic!(e);
            }
        }
    }

    let mut file = File::create(filename).expect("Could not open result file");
    file.write_all(out_str.as_bytes()).expect("Could not write results");
}

/// Clean the current line. Used for the progressbar effect.
fn clean_line() {
    print!("\r                                                                ");
    io::stdout().flush().ok().expect("Could not flush stdout");
}
