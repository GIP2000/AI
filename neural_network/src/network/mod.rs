mod activation;
mod loss;

use activation::{sig as g, sig_prime as g_prime};
use anyhow::Result;
use std::iter::zip;
use std::{fs::OpenOptions, io::Write};

const BIAS: f64 = -1f64;

#[derive(Debug, Clone)]
pub struct Metric {
    correct_positive: u32,
    correct_negative: u32,
    false_positive: u32,
    false_negative: u32,
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            correct_positive: 0,
            correct_negative: 0,
            false_positive: 0,
            false_negative: 0,
        }
    }
}

impl Metric {
    fn update(&mut self, (real, pred): (u8, u8)) {
        match (real, pred) {
            (1, 1) => self.correct_positive += 1,
            (0, 0) => self.correct_negative += 1,
            (1, 0) => self.false_negative += 1,
            (0, 1) => self.false_positive += 1,
            _ => {
                panic!("Error all values must be 0 or 1")
            }
        }
    }
}

pub struct Network {
    layers: Vec<Vec<Vec<f64>>>,
    input_size: usize,
}

impl Network {
    pub fn new(shape: Vec<usize>, lines: Vec<Vec<f64>>) -> Self {
        let mut line_iter = lines.into_iter();
        Self {
            input_size: shape[0],
            layers: shape
                .iter()
                .skip(1)
                .map(|&x| {
                    (0..x)
                        .into_iter()
                        .map(|_| line_iter.next().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn save(&self, file_path: &String) -> Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        let mut first_line = vec![self.input_size.to_string()];
        for layer in self.layers.iter() {
            first_line.push(layer.len().to_string());
        }

        writeln!(&mut f, "{}", first_line.join(" "))?;

        for layer in self.layers.iter() {
            for node in layer.iter() {
                writeln!(
                    &mut f,
                    "{}",
                    node.iter()
                        .fold("".to_string(), |acc, v| format!("{} {}", acc, v))
                )?;
            }
        }
        return Result::Ok(());
    }

    pub fn train(&mut self, X: Vec<Vec<f64>>, Y: Vec<Vec<f64>>, epoch: u32, learning_rate: f64) {
        for e in 0..epoch {
            println!("Staring epoch {}", e);
            for (x, y) in zip(X.iter(), Y.iter()) {
                // forward prop
                let (a, in_vec) = self.predict_float(x.clone());
                //back propigate
                let mut grad: Vec<f64> = (0..self.layers[self.layers.len() - 1].len())
                    .map(|j| {
                        return g_prime(in_vec[j]) * (y[j] - a[j]);
                    })
                    .collect();

                for (_, layer) in self.layers.iter().rev().skip(1).enumerate() {
                    // this is terrible for performance I could do this before likely
                    if grad.len() < layer.len() {
                        grad.resize(layer.len(), 0f64);
                    }
                    for (i, node) in layer.iter().enumerate() {
                        // this is terrible for performance I could do this before likely
                        if grad.len() < node.len() {
                            grad.resize(node.len(), 0f64);
                        }
                        grad[i] = g_prime(in_vec[i])
                            * node.iter().enumerate().fold(0f64, |acc, (j, w)| {
                                return acc + w * grad[j];
                            });
                    }
                }
                // update weights
                for (i, layer) in self.layers.iter_mut().enumerate() {
                    for node in layer.iter_mut() {
                        for (j, w) in node.iter_mut().enumerate() {
                            *w += learning_rate * a[i] * grad[j];
                        }
                    }
                }
            }
        }
    }

    fn predict_float(&self, mut x: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        // this seems wrong
        let mut a = vec![BIAS];
        a.append(&mut x);

        let mut in_vec = vec![];
        for layer in self.layers.iter() {
            if layer.len() > a.len() {
                a.resize(layer.len(), 0f64);
            }
            if layer.len() > in_vec.len() {
                in_vec.resize(layer.len(), 0f64);
            }
            for (j, node) in layer.iter().enumerate() {
                in_vec[j] = node.iter().fold(0f64, |acc, w| acc + (w * a[j]));
                a[j] = g(in_vec[j]);
            }
        }
        return (a, in_vec);
    }

    pub fn predict(&self, x: Vec<f64>) -> Vec<u8> {
        let l = x.len();
        return self
            .predict_float(x)
            .0
            .split_at(l)
            .0
            .iter()
            .map(|&x| {
                if x >= 0.5 {
                    return 1u8;
                }
                return 0u8;
            })
            .collect();
    }

    pub fn test(&self, X: Vec<Vec<f64>>, Y: Vec<Vec<u8>>) -> Vec<Metric> {
        let mut response: Vec<Metric> = vec![Metric::default(); Y[0].len()];
        for (x, y) in zip(X, Y) {
            let pred = self.predict(x);
            for (i, v) in zip(y, pred).enumerate() {
                response[i].update(v);
            }
        }
        return response;
    }
}
