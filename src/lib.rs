use console::{self, Alignment, Term};
use std::error::Error;
use std::io;

mod cat_art;
use cat_art::*;

/// The line width used when writing to a file.
const FILE_LINE_WIDTH: u16 = 80;

/// Returns a row of the cat padded to be centered in the terminal.
fn get_padded_row(row: &str, width: u16, symbol: Option<char>) -> String {
    let symbol = symbol.unwrap_or(' ');

    console::pad_str(row, width as usize, Alignment::Center, None)
        .to_string()
        .replace(' ', &symbol.to_string())
        + "\n"
}

/// Formats a part of the cat to be printed, using the index of the line
/// separating the text from the cat.
pub fn format_cat_part(part: &'static [&str], line: usize, line_char: char, width: u16) -> String {
    let mut formatted_part = String::new();
    for i in 0..part.len() {
        if i == line {
            formatted_part += &get_padded_row(part[i], width, Some(line_char));
        } else {
            formatted_part += &get_padded_row(part[i], width, None);
        }
    }

    formatted_part
}

/// Writes the lines from input in addition to the cat graphics.
pub fn write_output(input: impl Iterator<Item = io::Result<String>>) -> Result<(), Box<dyn Error>> {
    let width = match Term::stdout().size_checked() {
        Some((_, w)) => w,
        None => {
            // This probably means the user is trying to write to a file, so we
            // use an arbitrary default.
            FILE_LINE_WIDTH
        }
    };

    // We use `print!` instead of `println!` here because `format_cat_part`
    // adds an extra newline at the end.
    print!(
        "{}",
        format_cat_part(CAT_UPPER, CAT_UPPER.len() - 1, '─', width)
    );

    for line in input {
        println!("{}", line?);
    }

    println!("{}", format_cat_part(CAT_LOWER, 0, '─', width));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::get_padded_row;

    #[test]
    fn row_padding() {
        let padded_row = get_padded_row("hello", 15, Some('-'));
        assert_eq!(
            padded_row,
            "-----hello-----\n".to_string(),
            "`get_padded_row` was not correct, got `{}`",
            padded_row,
        );
    }
}
