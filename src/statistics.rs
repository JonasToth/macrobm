/// Calculate statistics from the measured durations

extern crate stat;
use stat::{mean, minmax, absdev};
use std::collections::{HashMap};
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

pub fn process_results(run_statistic: &HashMap<String, Vec<f32>>) {

    println!("{:10} {:8} {:7} {:7} {:20}", Blue.bold().paint("Avg"), Blue.bold().paint("Dev"),
                               Blue.bold().paint("Min"), Blue.bold().paint("Max"),
                               Blue.bold().paint("Name"));

    for bm_name in run_statistic.keys() {
        let ref times = run_statistic.get(bm_name).unwrap();
        let avg = mean(times);
        let (min, _, max, _) = minmax(times);
        let dev = absdev(times);
        let reldev = dev / avg * 100.;

        println!("{:8.2} {:3.1}% {:8.2} {:8.2} {}", Bold.paint(avg), reldev, min, 
                                                    max, Bold.paint(bm_name));
    }
}
