use anyhow::{Context, Result};
use std::{fs::read_to_string, str::FromStr};

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

    let mut sizes = vec![];
    for (&cur, &prev) in first_row.iter().skip(1).zip(first_row.iter()) {
        for _ in 0..prev {
            sizes.push(cur);
        }
    }

    let mut x = vec![];
    let mut y = vec![];

    for (line, size) in std::iter::zip(lines, sizes) {
        if line.as_bytes().is_empty() {
            continue;
        }
        x.push(delim_parse(
            line.split(' ')
                .enumerate()
                .filter(|&(i, _)| i < size)
                .map(|(_, s)| s),
        )?);
        y.push(delim_parse(line.split(' ').skip(size))?);
    }
    return Result::Ok((x, y));
}
