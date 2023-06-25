use colored_char::{line_to_colored_chars, print_chars, ColoredChar};

mod colored_char;
mod diff;
mod io;

fn main() {
    let (left, right) = match io::files() {
        Ok(v) => v,
        Err(err) => {
            println!("{err}");
            std::process::exit(1);
        }
    };

    let lines = diff::diff(&left, &right);

    let mut chars: Vec<ColoredChar> = lines
        .map(|(line_number, line)| line_to_colored_chars(line_number, line))
        .map(|zipped_lines| -> Vec<ColoredChar> {
            let (left, right): (Vec<_>, Vec<_>) = zipped_lines.unzip();
            left.into_iter()
                .chain(right.into_iter())
                .flatten()
                .collect()
        })
        .flatten()
        .collect();

    if chars.len() > 0 {
        chars.pop().expect("pop newline");
    }

    print_chars(chars);
}
