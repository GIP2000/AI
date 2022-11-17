use std::error::Error;
use std::{fs::read_to_string, str::FromStr};

use anyhow::{Context, Result};

pub fn parse_weight_file(file_name: &String) -> Result<(Vec<usize>, Vec<Vec<f64>>)> {
    let s = read_to_string(file_name)?;
    let mut lines = s.split('\n');
    let shape = delim_parse(lines.next().context("no shape found")?.split(' '))?;
    let weights: Vec<Vec<f64>> = lines
        .map(|l| delim_parse::<f64>(l.split(' ')))
        .collect::<Result<Vec<Vec<f64>>>>()?;
    return Result::Ok((shape, weights));
}

fn delim_parse<'a, T: FromStr>(line: impl Iterator<Item = &'a str>) -> Result<Vec<T>> {
    let b = line
        .filter(|v| !v.as_bytes().is_empty())
        .map(|v| {
            v.parse::<T>()
                .ok()
                .with_context(|| format!("could not parse **{}**", v))
        })
        .collect::<Result<Vec<T>>>();
    return b;
}

pub fn parse_data_file<T: FromStr>(file_name: &String) -> Result<(Vec<Vec<f64>>, Vec<Vec<T>>)> {
    let fs = read_to_string(file_name)?;
    let mut lines = fs.split('\n');
    let first_row: Vec<usize> =
        delim_parse(lines.next().context("no first_row found")?.split(' '))?;
    let input_size = *first_row.get(1).context("no input size found")?;

    let mut x = vec![];
    let mut y = vec![];

    for line in lines {
        if line.as_bytes().is_empty() {
            continue;
        }
        x.push(delim_parse(
            line.split(' ')
                .enumerate()
                .filter(|&(i, _)| i < input_size)
                .map(|(_, s)| s),
        )?);
        y.push(delim_parse(line.split(' ').skip(input_size))?);
    }
    return Result::Ok((x, y));
}
