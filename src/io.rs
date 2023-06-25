use std::{env, fs};

fn operands() -> Result<(String, String), String> {
    let mut args = env::args();
    let name = args
        .next()
        .ok_or("cdiff: unreachable: called process without process name".to_string())?;
    let left = args
        .next()
        .ok_or(format!("cdiff: missing operand after '{name}'"))?;
    let right = args
        .next()
        .ok_or(format!("cdiff: missing operand after '{left}'"))?;
    args.next().map_or(Ok(()), |extra| {
        Err(format!("cdiff: extra operand '{extra}'"))
    })?;

    Ok((left, right))
}

pub fn files() -> Result<(String, String), String> {
    let (left, right) = operands()?;

    let left = fs::read_to_string(&left)
        .map_err(|_| format!("cdiff: '{left}': no such file or directory"))?;

    let right = fs::read_to_string(&right)
        .map_err(|_| format!("cdiff: '{right}': no such file or directory"))?;

    Ok((left, right))
}
