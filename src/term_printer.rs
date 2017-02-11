extern crate rustbox; 
use rustbox::{Color, RustBox};
use rustbox::Key;



pub fn print_control_message(tui: &RustBox) {
    tui.print(1, 1, rustbox::RB_BOLD, Color::White, Color::Default,
    "Running macro benchmarks");
}
