use console::{pad_str, Alignment};

/// Returns a row of the cat padded to be centered in the terminal.
fn get_padded_row(row: &str, width: u16, symbol: Option<char>) -> String {
    let symbol = symbol.unwrap_or(' ');
    return pad_str(row, width as usize, Alignment::Center, None)
        .to_string()
        .replace(' ', &symbol.to_string());
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
    return formatted_part;
}

#[cfg(test)]
mod tests {
    use crate::get_padded_row;

    #[test]
    fn row_padding() {
        let padded_row = get_padded_row("hello", 15, Some('-'));
        assert_eq!(
            padded_row, "-----hello-----".to_string(),
            "`get_padded_row` was not correct, got `{}`", padded_row,
        );
    }
}
