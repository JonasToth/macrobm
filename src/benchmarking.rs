/// Logic and functionality to actually perform the benchmarks.
///

// subprocesses to call the command we want to measure
use std::process::{Command, Stdio, ExitStatus};

// parallelism
use threadpool::ThreadPool;
use std::sync::mpsc::Sender;

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

/// Start all benchmarks in a threadpool and configure a channel to receive a Report for every
/// finished run.
pub fn do_benchmark(pool: &ThreadPool,
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
            let mut process = Command::new(&cmd);
            process.args(&args)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .current_dir(&dir);

            match process.spawn() {
                Ok(mut child) => {
                    match child.wait() {
                        Ok(ecode) =>  {
                            let execution_time = start_time.elapsed();
                            tx.send(Report::new(name, execution_time, ecode)).unwrap();
                        }
                        Err(_) => {
                            println!("Program did not finish as expected");
                            println!("Command: {}", &cmd);
                            println!("Workdir: {}", &dir);
                        }
                    }
                }
                Err(_) => {
                    println!("Could not start benchmark {}", &name);
                    println!("Command: {}", &cmd);
                    println!("Workdir: {}", &dir);
                }
            }
        });
    }
}


// --------------------- tests for the functionality of benchmarking ---------------------------
#[test]
fn test_duration_conversion_to_seconds_float() {
    let d = Duration::from_millis(1500);
    assert_eq!(convert_duration_to_seconds(d), 1.5);
}
