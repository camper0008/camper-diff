use itertools::{EitherOrBoth, Itertools};

#[derive(Debug)]
pub enum Diff<T> {
    Same(T),
    Different(T, T),
}
/*
fn format_diff<'a>(left: &'a str, right: &'a str) -> (Vec<ColoredString>, Vec<ColoredString>) {
    let chars = merge(diff::chars(left, right));

    let left = chars
        .iter()
        .map(|result| match result {
            Diff::Different(left, _right) => {
                left.unwrap_or(' ').to_string().black().on_red().bold()
            }
            Diff::Same(Some(c)) => c.to_string().red(),
            Diff::Same(None) => unreachable!(),
        })
        .collect();

    let right = chars
        .iter()
        .map(|result| match result {
            Diff::Different(_left, right) => {
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
    Ø*/

pub fn char_diff<'a>(left: Option<&'a str>, right: Option<&'a str>) -> Vec<Diff<Option<char>>> {
    let (left, right) = match (left, right) {
        (None, Some(right)) => {
            return right
                .chars()
                .map(|char| Diff::Different(None, Some(char)))
                .collect()
        }
        (Some(left), None) => {
            return left
                .chars()
                .map(|char| Diff::Different(Some(char), None))
                .collect()
        }
        (Some(left), Some(right)) => (left, right),
        (None, None) => unreachable!("should cull empty lines"),
    };
    let left = left.chars();
    let right = right.chars();

    left.zip_longest(right)
        .map(|char| match char {
            EitherOrBoth::Both(left, right) if left == right => Diff::Same(Some(left)),
            EitherOrBoth::Both(left, right) => Diff::Different(Some(left), Some(right)),
            EitherOrBoth::Left(left) => Diff::Different(Some(left), None),
            EitherOrBoth::Right(right) => Diff::Different(None, Some(right)),
        })
        .collect()
}

pub fn line_diff<'a>(
    left: &'a str,
    right: &'a str,
) -> impl Iterator<Item = (usize, Diff<Option<&'a str>>)> {
    let left = left.lines();
    let right = right.lines();

    left.zip_longest(right)
        .map(|line| match line {
            EitherOrBoth::Both(left, right) if left == right => Diff::Same(Some(left)),
            EitherOrBoth::Both(left, right) => Diff::Different(Some(left), Some(right)),
            EitherOrBoth::Left(left) => Diff::Different(Some(left), None),
            EitherOrBoth::Right(right) => Diff::Different(None, Some(right)),
        })
        .enumerate()
        .map(|(line_number, diff)| (line_number + 1, diff))
}

pub fn diff<'a>(
    left: &'a str,
    right: &'a str,
) -> impl Iterator<Item = (usize, Vec<Diff<Option<char>>>)> + 'a {
    let lines = line_diff(left, right);
    lines.filter_map(|(line_number, diff)| match diff {
        Diff::Same(_) => None,
        Diff::Different(left, right) => Some((line_number, char_diff(left, right))),
    })
}
