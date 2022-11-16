use std::{fs::read_to_string, str::FromStr};

pub fn parse_weight_file(file_name: &String) -> Option<(Vec<usize>, Vec<Vec<f64>>)> {
    let s = read_to_string(file_name).ok()?;
    let mut lines = s.split('\n');
    let shape = delim_parse(lines.next()?.split(' '))?;
    let weights: Vec<Vec<f64>> = lines
        .map(|l| delim_parse::<f64>(l.split(' ')))
        .collect::<Option<Vec<Vec<f64>>>>()?;
    return Some((shape, weights));
}

fn delim_parse<'a, T: FromStr>(line: impl Iterator<Item = &'a str>) -> Option<Vec<T>> {
    return line
        .map(|v| v.parse::<T>().ok())
        .collect::<Option<Vec<T>>>();
}

pub fn parse_data_file<T: FromStr>(file_name: &String) -> Option<(Vec<Vec<f64>>, Vec<Vec<T>>)> {
    let fs = read_to_string(file_name).ok()?;
    let mut lines = fs.split('\n');
    let first_row: Vec<usize> = delim_parse(lines.next()?.split(' '))?;
    let input_size = *first_row.get(1)?;

    let mut x = vec![];
    let mut y = vec![];

    for line in lines {
        x.push(delim_parse(
            line.split(' ')
                .enumerate()
                .filter(|&(i, _)| i < input_size)
                .map(|(_, s)| s),
        )?);
        y.push(delim_parse(line.split(' ').skip(input_size))?);
    }
    return Some((x, y));
}
