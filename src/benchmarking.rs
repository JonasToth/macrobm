/// Logic and functionality to actually perform the benchmarks.
///

use bm_runconfig::RunConfig;

// subprocesses to call the command we want to measure
use std::process::{Command, Stdio, ExitStatus};
use threadpool::ThreadPool;
use std::sync::mpsc;
use std::time::{Instant, Duration};

/// Data one benchmark run produces.
pub struct Report {
    pub name: String,
    pub duration: f32,
    pub ecode: ExitStatus,
}

impl Report {
    pub fn new(name: String, dur: Duration, code: ExitStatus) -> Report {
        let seconds: f32 = dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1000000000.;
        Report {
            name: name,
            duration: seconds,
            ecode: code,
        }
    }
}
pub fn do_benchmark(pool: &ThreadPool, name: &str, channel_trans: mpsc::Sender<Report>, config: &RunConfig) {
    for _ in 0..config.count {
        // threads need own version of the data
        let name = name.to_string();
        let cmd = config.command.clone();
        let args = config.args.clone();
        let tx = channel_trans.clone();
        let dir = config.directory.clone();
        //let env = config.environment.clone();

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
                               .expect("Failed to wait on program!");
            let execution_time = start_time.elapsed();
            tx.send(Report::new(name, execution_time, ecode)).unwrap();
        });
    }
}
