use catcat::format_cat_part;
use std::{env, fs};
use terminal_size::{terminal_size, Width};

mod cat_art;
use cat_art::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let text = fs::read_to_string(filename).expect("Error reading file");

    if let Some((Width(width), _)) = terminal_size() {
        println!(
            "{}",
            format_cat_part(CAT_UPPER, CAT_UPPER.len() - 1, '─', width)
        );
        println!("{}", text.trim());
        println!("{}", format_cat_part(CAT_LOWER, 0, '─', width));
    }
}
