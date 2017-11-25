/// Logic and functionality to actually perform the benchmarks.
///

use messages;

// results and configuration
use std::collections::BTreeMap;

// subprocesses to call the command we want to measure
use std::process::{Command, Stdio, ExitStatus};

// parallelism
use threadpool::ThreadPool;
use std::sync::mpsc::{Sender, Receiver};

// time measurements
use std::time::{Instant, Duration};

/// Define values used to configure a benchmark run.
#[derive(Debug)]
pub struct RunConfig {
    pub name: String,
    pub description: String,
    pub count: i64,

    pub command: String,
    pub args: Vec<String>, // empty vector if no args were configured
    pub directory: String, // optional
    pub environment: Vec<String>, // optional
}

/// Data one benchmark run produces.
#[derive(Debug)]
pub struct Report {
    pub name: String,
    pub duration: f32,
    pub ecode: ExitStatus,
}

impl Report {
    pub fn new(name: String, dur: Duration, code: ExitStatus) -> Report {
        Report {
            name: name,
            duration: convert_duration_to_seconds(dur),
            ecode: code,
        }
    }
}

fn convert_duration_to_seconds(dur: Duration) -> f32 {
    dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1000000000.
}

/// This function schedules all benchmarks that are supposed to run
/// several times and distributes them over `n_workers` threads.
pub fn schedule_benchmarks(bm_cfg: BTreeMap<String, RunConfig>,
                           n_workers: usize,
                           tx: Sender<Report>
                          ) -> i64 {
    // --------------- Banner Message
    messages::intro(n_workers);

    let pool = ThreadPool::new(n_workers);
    let mut scheduled = 0;

    for (name, config) in &bm_cfg {
        messages::scheduled_command(&name, config.count);
        do_benchmark(&pool, &name, tx.clone(), config);
        scheduled += config.count;
    }
    scheduled
}

/// Collect all results for the benchmarks that were scheduled and return
/// the statistical data.
pub fn collect_results(scheduled: i64, rx: Receiver<Report>
                      ) -> (BTreeMap<String, Vec<f32>>, i64, i64) {
    let mut stats = BTreeMap::<String, Vec<f32>>::new();

    // ------------- Wait for all bm to finish and notice the user about the state of the program.
    let mut successes = 0;
    let mut fails = 0;

    for finished in 0..scheduled {
        let report = rx.recv().unwrap();

        // process report
        stats.entry(report.name.clone()).or_insert(Vec::<f32>::new())
            .push(report.duration);

        // output information
        messages::finished_program(&report, finished + 1, scheduled);

        if report.ecode.success() {
            successes += 1
        } else {
            fails += 1
        }
    }
    messages::finished();
    (stats, successes, fails)
}

/// Start all benchmarks in a threadpool and configure a channel to receive a Report for every
/// finished run.
fn do_benchmark(pool: &ThreadPool,
                name: &str,
                channel_trans: Sender<Report>,
                config: &RunConfig) {
    for _ in 0..config.count {
        // threads need own version of the data
        let name = name.to_string();
        let cmd = config.command.clone();
        let args = config.args.clone();
        let tx = channel_trans.clone();
        let dir = config.directory.clone();
        // let env = config.environment.clone();

        pool.execute(move || {
            let start_time = Instant::now();
            let mut process = Command::new(&cmd)
                .args(&args)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .current_dir(&dir)
                .spawn()
                .expect("Program start failed!");
            let ecode = process.wait()
                .expect("Failed to wait on program to finish!");
            let execution_time = start_time.elapsed();
            tx.send(Report::new(name, execution_time, ecode)).unwrap();
        });
    }
}


// --------------------- tests for the functionality of benchmarking ---------------------------
#[test]
fn test_duration_conversion_to_seconds_float() {
    let d = Duration::from_millis(1500);
    assert_eq!(convert_duration_to_seconds(d), 1.5);
}
