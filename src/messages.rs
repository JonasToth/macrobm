extern crate term_painter;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

pub fn intro(worker: usize) {
    println!("Running {} {} threads", Bold.paint("macro benchmarks"), worker);
}

pub fn scheduled_command(name: String, count: i64) {
    println!("Scheduling {} for {} runs", Bold.paint(name), Bold.paint(count));
}

pub fn finished_program(report: u64)
{
    // err_code.success()
    let state_string = if true { Green.bold().paint("Success") } 
                       else { Red.bold().paint("Failed") };
    let execution_time_seconds = report;

    println!("{} {} after {} seconds", state_string, Bold.paint("DummyRightNow"), 
                                       Bold.paint(execution_time_seconds));
}

pub fn finished() {
    println!("Finished running benchmarks.!");
}
