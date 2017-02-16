extern crate term_painter;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use execution_report::Report;

pub fn intro(worker: usize) {
    println!("Running {} {} threads", Bold.paint("macro benchmarks"), worker);
}

pub fn intro_report() {
    println!("");
    println!("");
}

pub fn invalid_config_filename(fname: &str) {
    println!("{} could not open file {} as config.", Red.bold().paint("Failure"), Red.paint(fname));
}

pub fn scheduled_command(name: &str, count: i64) {
    println!("{} {} for {} runs", Blue.paint("Scheduling"), Bold.paint(name), 
                                  Bold.paint(count));
}

pub fn finished_program(report: Report)
{
    // err_code.success()
    let state_string = if report.ecode.success() { Green.bold().paint("Success") } 
                       else { Red.bold().paint("Failure") };
    let exec_time = report.duration;

    println!("{} {} after {:.2} {}", state_string, Bold.paint(report.name), 
                                      Bold.paint(exec_time), Bold.paint("seconds"));
}

pub fn finished() {
    println!("{}", Green.bold().paint("Finished running benchmarks.!"));
}

