mod activation;
mod loss;

use activation::sig as g;
use anyhow::{Context, Result};
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

pub struct Node {
    pub a: f64,
    pub grad: f64,
    pub prev_weights: Vec<f64>,
}

impl Default for Node {
    fn default() -> Self {
        return Self::new(BIAS, 0f64, vec![]);
    }
}

impl Node {
    pub fn new(a: f64, grad: f64, prev_weights: Vec<f64>) -> Self {
        return Self {
            a,
            grad,
            prev_weights,
        };
    }

    pub fn prime(&self) -> f64 {
        return self.a * (1f64 - self.a);
    }
}

pub struct Network {
    layers: Vec<Vec<Node>>,
    input_size: usize,
}

impl std::fmt::Debug for Network {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (l, layer) in self.layers.iter().enumerate() {
            for (n, node) in layer.iter().enumerate() {
                writeln!(fmt, "{:?} {:?} {:?}", l, n, node.prev_weights)?;
            }
        }
        return Result::Ok(());
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut first_line = vec![self.input_size.to_string()];
        for layer in self.layers.iter() {
            first_line.push((layer.len() - 1).to_string());
        }

        writeln!(fmt, "{}", first_line.join(" "))?;

        for layer in self.layers.iter() {
            for node in layer.iter().skip(1) {
                writeln!(fmt, "{}", {
                    let mut a = node
                        .prev_weights
                        .iter()
                        .fold("".to_string(), |acc, v| format!("{}{} ", acc, v));
                    a.pop();
                    a
                })?;
            }
        }
        return Ok(());
    }
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
                    let mut result = Vec::with_capacity(x + 1);
                    result.push(Node::default());
                    for _ in 0..x {
                        result.push(Node::new(
                            0f64,
                            0f64,
                            line_iter.next().expect("Expected another node"),
                        ));
                    }
                    return result;
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

        return write!(&mut f, "{}", self).context("Error Writting");
    }

    pub fn train(&mut self, X: Vec<Vec<f64>>, Y: Vec<Vec<f64>>, epoch: u32, learning_rate: f64) {
        for e in 0..epoch {
            println!("Staring epoch {}", e);
            for (x, y) in zip(X.iter(), Y.iter()) {
                // forward prop
                self.predict_float(x.clone());

                //back propigate

                // inital grads
                for (node, y_val) in zip(
                    self.layers
                        .last_mut()
                        .expect("Error no layers")
                        .iter_mut()
                        .skip(1),
                    y.iter(),
                ) {
                    node.grad = node.prime() * (y_val - node.a);
                }

                // back propigated gardiants
                for l in (0..(self.layers.len() - 1)).rev() {
                    for i in 1..self.layers[l].len() {
                        self.layers[l][i].grad = self.layers[l][i].prime()
                            * (1..self.layers[l + 1].len()).fold(0f64, |acc, j| {
                                acc + self.layers[l][i].prev_weights[j] * self.layers[l + 1][j].grad
                            });
                    }
                }

                // update weights
                for l in (0..(self.layers.len())).rev() {
                    for n in 0..self.layers[l].len() {
                        for w in 0..self.layers[l][n].prev_weights.len() {
                            self.layers[l][n].prev_weights[w] += learning_rate
                                * if l == 0 {
                                    BIAS
                                } else {
                                    self.layers[l - 1][w].a
                                }
                                * self.layers[l][n].grad;
                        }
                    }
                }
            }
        }
    }

    fn predict_float(&mut self, a: Vec<f64>) {
        for node in self.layers[0].iter_mut() {
            node.a = g(node
                .prev_weights
                .iter()
                .skip(1)
                .enumerate()
                .fold(0f64, |acc, (wi, w)| acc + w * a[wi]))
        }

        for l in 1..self.layers.len() {
            for j in 1..self.layers[l].len() {
                self.layers[l][j].a = g((0..self.layers[l][j].prev_weights.len())
                    .fold(0f64, |acc, w| {
                        acc + self.layers[l][j].prev_weights[w] * self.layers[l - 1][w].a
                    }));
            }
        }
    }

    pub fn predict(&mut self, a: Vec<f64>) -> Vec<u8> {
        self.predict_float(a);
        return self.layers[self.layers.len() - 1]
            .iter()
            .map(|x| {
                if x.a >= 0.5 {
                    return 1u8;
                }
                return 0u8;
            })
            .collect::<Vec<_>>();
    }

    pub fn test(&mut self, X: Vec<Vec<f64>>, Y: Vec<Vec<u8>>) -> Vec<Metric> {
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
