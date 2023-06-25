use colored::ColoredString;
use colored::Colorize;

#[derive(Debug)]
pub enum Diff<T> {
    Same(T),
    Different { left: T, right: T },
}

fn diff_line_with_ln<'a, I: Iterator<Item = diff::Result<&'a str>>>(
    iter: I,
) -> Vec<(usize, Diff<&'a str>)> {
    let mut left_bucket: Vec<&'a str> = Vec::new();
    let mut right_bucket: Vec<&'a str> = Vec::new();

    iter.for_each(|result| match result {
        diff::Result::Both(left, right) => {
            left_bucket.push(left);
            right_bucket.push(right);
        }
        diff::Result::Left(char) => {
            left_bucket.push(char);
        }
        diff::Result::Right(char) => {
            right_bucket.push(char);
        }
    });

    left_bucket
        .into_iter()
        .zip(right_bucket.into_iter())
        .map(|(left, right)| {
            if left == right {
                Diff::Same(left)
            } else {
                Diff::Different { left, right }
            }
        })
        .enumerate()
        .map(|(ln, diff)| (ln + 1, diff))
        .collect()
}

fn merge_results<I: Iterator<Item = diff::Result<char>>>(iter: I) -> Vec<Diff<Option<char>>> {
    let mut left_bucket: Vec<char> = Vec::new();
    let mut right_bucket: Vec<char> = Vec::new();

    iter.for_each(|result| match result {
        diff::Result::Both(left, right) => {
            left_bucket.push(left);
            right_bucket.push(right);
        }
        diff::Result::Left(char) => {
            left_bucket.push(char);
        }
        diff::Result::Right(char) => {
            right_bucket.push(char);
        }
    });

    let mut bucket = Vec::new();

    let mut left_bucket = left_bucket.into_iter();
    let mut right_bucket = right_bucket.into_iter();

    loop {
        let left = left_bucket.next();
        let right = right_bucket.next();
        let diff = match (left, right) {
            (Some(left), Some(right)) if left != right => Diff::Different {
                left: Some(left),
                right: Some(right),
            },
            (Some(_), None) | (None, Some(_)) => Diff::Different { left, right },
            (Some(c), Some(_)) => Diff::Same(Some(c)),
            (None, None) => break bucket,
        };
        bucket.push(diff)
    }
}

fn format_diff<'a>(left: &'a str, right: &'a str) -> (Vec<ColoredString>, Vec<ColoredString>) {
    let chars = diff::chars(left, right);
    let chars = merge_results(chars.into_iter());

    let left = chars
        .iter()
        .map(|result| match result {
            Diff::Different { left, right: _ } => {
                left.unwrap_or(' ').to_string().black().on_red().bold()
            }
            Diff::Same(Some(c)) => c.to_string().red(),
            Diff::Same(None) => unreachable!(),
        })
        .collect();

    let right = chars
        .iter()
        .map(|result| match result {
            Diff::Different { left: _, right } => {
                if let Some(right) = right {
                    right.to_string().black().on_green().bold()
                } else {
                    String::new().black()
                }
            }
            Diff::Same(Some(c)) => c.to_string().green(),
            Diff::Same(None) => unreachable!(),
        })
        .collect();

    (left, right)
}

fn main() {
    let left = "aaaaa\n000aaa000\nbbbaaaCCC";
    let right = "aaaaa\n000bbb000\naaaaaa";

    let lines = diff::lines(left, right).into_iter();
    let lines = diff_line_with_ln(lines);

    let mut lines = lines
        .into_iter()
        .filter_map(|(ln, diff)| match diff {
            Diff::Same(_) => None,
            Diff::Different { left, right } => Some((ln, format_diff(left, right))),
        })
        .peekable();

    loop {
        let Some((ln, (left, right))) = lines.next() else {
            break
        };
        print!("{}: ", ln.to_string().yellow());
        left.into_iter().for_each(|c| print!("{}", c));
        println!();

        print!("{}: ", ln.to_string().yellow());
        right.into_iter().for_each(|c| print!("{}", c));
        println!();

        if lines.peek().is_some() {
            println!();
        }
    }
}
