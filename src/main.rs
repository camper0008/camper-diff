// TODO: use colored-char

use colored::Colorize;

mod difference;
mod io;

fn main() {
    let (left, right) = match io::files() {
        Ok(v) => v,
        Err(err) => {
            println!("{err}");
            std::process::exit(1);
        }
    };

    let mut lines = difference::difference(&left, &right).peekable();

    loop {
        let Some((ln, (left, right))) = lines.next() else {
            break
        };
        print!("{}: ", ln.to_string().yellow());
        for c in left {
            print!("{c}");
        }
        println!();

        print!("{}: ", ln.to_string().yellow());
        for c in right {
            print!("{c}");
        }
        println!();

        if lines.peek().is_some() {
            println!();
        }
    }
}
