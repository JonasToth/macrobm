/// Contains information(duration, error_code,..) about one run of a benchmark.
///
///

use std::time::Duration;
use std::process::ExitStatus;

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
