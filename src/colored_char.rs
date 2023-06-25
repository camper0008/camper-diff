use colored::Colorize;

use crate::diff::Diff;

#[derive(Debug, PartialEq)]
pub enum ColoredChar {
    Unimportant(char),
    LineNumber(char),
    Same(char),
    HookLeft(char),
    HookRight(char),
    DifferentLeft(char),
    DifferentRight(char),
    Space,
    Newline,
    Blank,
}

impl ColoredChar {
    fn wrap(self) -> Vec<Self> {
        vec![self]
    }
}

pub fn line_to_colored_chars(
    line_number: usize,
    line: Vec<Diff<Option<char>>>,
) -> impl Iterator<Item = (Vec<ColoredChar>, Vec<ColoredChar>)> {
    let left_buffer = prepare_buffer(line_number, ColoredChar::HookLeft('<')).into_iter();
    let right_buffer = prepare_buffer(line_number, ColoredChar::HookRight('>')).into_iter();
    let mut empty_line_should_appear = true;

    let chars = line.into_iter().map(move |diff| match diff {
        Diff::Same(Some(char)) => (
            ColoredChar::Same(char).wrap(),
            ColoredChar::Same(char).wrap(),
        ),
        Diff::Different(Some(left), None) => (
            ColoredChar::Same(left).wrap(),
            empty_line_once(&mut empty_line_should_appear),
        ),
        Diff::Different(None, Some(right)) => (
            empty_line_once(&mut empty_line_should_appear),
            ColoredChar::Same(right).wrap(),
        ),
        Diff::Different(Some(left), Some(right)) => (
            ColoredChar::DifferentLeft(left).wrap(),
            ColoredChar::DifferentRight(right).wrap(),
        ),
        Diff::Same(None) | Diff::Different(None, None) => unreachable!(),
    });

    let newlines = vec![
        (ColoredChar::Newline.wrap(), ColoredChar::Newline.wrap()),
        (ColoredChar::Blank.wrap(), ColoredChar::Newline.wrap()),
    ]
    .into_iter();

    left_buffer.zip(right_buffer).chain(chars).chain(newlines)
}

fn prepare_buffer(line_number: usize, hook: ColoredChar) -> Vec<Vec<ColoredChar>> {
    let line_number = line_number.to_string();
    let line_number = line_number.chars().map(ColoredChar::LineNumber);

    let rest = vec![
        ColoredChar::Unimportant(':'),
        ColoredChar::Space,
        ColoredChar::Unimportant('('),
        hook,
        ColoredChar::Unimportant(')'),
        ColoredChar::Space,
    ]
    .into_iter();

    line_number.chain(rest).map(ColoredChar::wrap).collect()
}

pub fn print_chars(chars: Vec<ColoredChar>) {
    chars
        .into_iter()
        .map(|char| match char {
            ColoredChar::Unimportant(char) => char.to_string().white(),
            ColoredChar::LineNumber(char) => char.to_string().yellow(),
            ColoredChar::Same(char) => char.to_string().bright_white(),
            ColoredChar::HookLeft(char) => char.to_string().red(),
            ColoredChar::HookRight(char) => char.to_string().green(),
            ColoredChar::DifferentLeft(char) => char.to_string().on_red().black(),
            ColoredChar::DifferentRight(char) => char.to_string().on_green().black(),
            ColoredChar::Space => " ".white(),
            ColoredChar::Newline => "\n".white(),
            ColoredChar::Blank => "".white(),
        })
        .for_each(|char| print!("{char}"));
}

fn empty_line_once(should_appear: &mut bool) -> Vec<ColoredChar> {
    if *should_appear {
        *should_appear = false;
        "empty line".chars().map(ColoredChar::Unimportant).collect()
    } else {
        vec![]
    }
}
