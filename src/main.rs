use camper_diff::colored_char::{line_to_colored_chars, print_chars, ColoredChar};
use camper_diff::diff;
use camper_diff::io;

fn remove_last_newline(chars: &mut Vec<ColoredChar>) {
    if !chars.is_empty() {
        let char = chars.pop().expect("length > 0");
        assert_eq!(char, ColoredChar::Newline);
    }
}

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
        .flat_map(|zipped_lines| -> Vec<ColoredChar> {
            let (left, right): (Vec<_>, Vec<_>) = zipped_lines.unzip();
            left.into_iter()
                .chain(right.into_iter())
                .flatten()
                .collect()
        })
        .collect();
    remove_last_newline(&mut chars);
    print_chars(chars);
}
