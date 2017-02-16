/// Contains information(duration, error_code,..) about one run of a benchmark.
///
///

use std::time::Duration;
use std::process::ExitStatus;

pub struct Report {
    pub name: String,
    pub duration: f32,
    pub ecode: ExitStatus,
}

impl Report {
    pub fn new(name: &str, dur: Duration, code: ExitStatus) -> Report {
        let seconds: f32 = dur.as_secs() as f32 + dur.subsec_nanos() as f32 / 1000000000.;
        Report {
            name: name.to_string(),
            duration: seconds,
            ecode: code,
        }
    }
}
