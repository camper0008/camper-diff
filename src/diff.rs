use itertools::{EitherOrBoth, Itertools};

#[derive(Debug)]
pub enum Diff<T> {
    Same(T),
    Different(T, T),
}

fn chars<'a>(left: Option<&'a str>, right: Option<&'a str>) -> Vec<Diff<Option<char>>> {
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

fn lines<'a>(
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
    let lines = lines(left, right);
    lines.filter_map(|(line_number, diff)| match diff {
        Diff::Same(_) => None,
        Diff::Different(left, right) => Some((line_number, chars(left, right))),
    })
}
