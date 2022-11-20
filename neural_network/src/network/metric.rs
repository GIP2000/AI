use anyhow::Result;
use std::fmt::{Display, Formatter};
use std::{fs::OpenOptions, io::Write};

#[derive(Debug, Clone)]
pub struct Metric {
    A: f64,
    D: f64,
    B: f64,
    C: f64,
    is_global: bool,
}

impl Display for Metric {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if !self.is_global {
            return write!(
                fmt,
                "{} {} {} {} {:.3} {:.3} {:.3} {:.3}",
                self.A,
                self.B,
                self.C,
                self.D,
                self.accuracy(),
                self.percision(),
                self.recall(),
                self.f1()
            );
        }
        return write!(
            fmt,
            "{:.3} {:.3} {:.3} {:.3}",
            self.accuracy(),
            self.percision(),
            self.recall(),
            self.f1()
        );
    }
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            A: 0f64,
            D: 0f64,
            B: 0f64,
            C: 0f64,
            is_global: false,
        }
    }
}

impl std::ops::Add<Metric> for Metric {
    type Output = Metric;
    fn add(mut self, rhs: Metric) -> Self {
        self.A += rhs.A;
        self.B += rhs.B;
        self.C += rhs.C;
        self.D += rhs.D;
        self.is_global = true;
        return self;
    }
}

impl Metric {
    pub fn save(metrics: Vec<Metric>, file_path: &String) -> Result<()> {
        #[derive(Default)]
        struct Average {
            accuracy: f64,
            percision: f64,
            recall: f64,
        }

        impl Display for Average {
            fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(
                    fmt,
                    "{:.3} {:.3} {:.3} {:.3}",
                    self.accuracy,
                    self.percision,
                    self.recall,
                    self.f1()
                )
            }
        }

        impl Average {
            pub fn f1(&self) -> f64 {
                let percision = self.percision;
                let recall = self.recall;
                return (2f64 * percision * recall) / (percision + recall);
            }
            pub fn average(&mut self, div: f64) {
                self.accuracy /= div;
                self.percision /= div;
                self.recall /= div;
            }
        }

        impl From<Metric> for Average {
            fn from(metric: Metric) -> Self {
                Self {
                    accuracy: metric.accuracy(),
                    percision: metric.percision(),
                    recall: metric.recall(),
                }
            }
        }

        impl std::ops::Add<Average> for Average {
            type Output = Average;
            fn add(mut self, rhs: Self) -> Self {
                self.accuracy += rhs.accuracy;
                self.percision += rhs.percision;
                self.recall += rhs.recall;
                return self;
            }
        }

        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        let l = metrics.len();

        let (mut avg, global_metric) = metrics.into_iter().fold::<Result<(Average, Metric)>, _>(
            Result::Ok((Average::default(), Metric::default())),
            |acc, x| {
                let (avg, global) = acc?;
                writeln!(&mut f, "{}", x)?;
                return Ok((avg + x.clone().into(), global + x));
            },
        )?;
        avg.average(l as f64);
        writeln!(&mut f, "{}", global_metric)?;
        writeln!(&mut f, "{}", avg)?;
        return Ok(());
    }
    pub fn update(&mut self, (real, pred): (u8, u8)) {
        match (real, pred) {
            (1, 1) => self.A += 1f64,
            (0, 0) => self.D += 1f64,
            (1, 0) => self.C += 1f64,
            (0, 1) => self.B += 1f64,
            _ => {
                panic!("Error all values must be 0 or 1")
            }
        }
    }

    fn accuracy(&self) -> f64 {
        return ((self.A + self.D) as f64) / ((self.A + self.B + self.C + self.D) as f64);
    }

    fn percision(&self) -> f64 {
        return (self.A as f64) / ((self.A + self.B) as f64);
    }

    fn recall(&self) -> f64 {
        return (self.A as f64) / ((self.A + self.C) as f64);
    }

    fn f1(&self) -> f64 {
        let percision = self.percision();
        let recall = self.recall();
        return (2f64 * percision * recall) / (percision + recall);
    }
}
