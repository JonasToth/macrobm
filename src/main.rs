// this program is about "macro benchmarking", in the sense that it will measure whole programm
// execution and is able to compare run times.
// Provide shell scripts that shall be measured!

extern crate clap; // command line parsing
use clap::App;

extern crate term_painter; // nice colorfull console output
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

fn main() {
    /*let matches = App::new("macrobm")
                       .version("v0.1-beta")
                       .author("Jonas Toth <jonas.toth@gmail.com>")
                       .get_matches();*/

    println!("{} - Timing programm execution", Bold.paint("MacroBM"));
    println!("{} - So nice timing", Green.paint("Nice Programm"));
}
