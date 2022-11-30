use anyhow::Result;
use std::fmt::{Display, Formatter};
use std::{fs::OpenOptions, io::Write};

trait Calculate {
    fn accuracy(&self) -> f64;
    fn percision(&self) -> f64;
    fn recall(&self) -> f64;
    fn f1(&self) -> f64 {
        let percision = self.percision();
        let recall = self.recall();
        return (2f64 * percision * recall) / (percision + recall);
    }
}

#[derive(Debug, Clone)]
pub struct Metric {
    A: f64,
    D: f64,
    B: f64,
    C: f64,
}

impl Display for dyn Calculate {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{:.3} {:.3} {:.3} {:.3}",
            self.accuracy(),
            self.percision(),
            self.recall(),
            self.f1()
        )
    }
}

impl Calculate for Metric {
    fn accuracy(&self) -> f64 {
        return ((self.A + self.D) as f64) / ((self.A + self.B + self.C + self.D) as f64);
    }

    fn percision(&self) -> f64 {
        return (self.A as f64) / ((self.A + self.B) as f64);
    }

    fn recall(&self) -> f64 {
        return (self.A as f64) / ((self.A + self.C) as f64);
    }
}

impl Display for Metric {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        return write!(fmt, "{} {} {} {} ", self.A, self.B, self.C, self.D);
    }
}

impl Default for Metric {
    fn default() -> Self {
        Self {
            A: 0f64,
            D: 0f64,
            B: 0f64,
            C: 0f64,
        }
    }
}

impl std::ops::Add<&Metric> for Metric {
    type Output = Metric;
    fn add(mut self, rhs: &Metric) -> Self {
        self.A += rhs.A;
        self.B += rhs.B;
        self.C += rhs.C;
        self.D += rhs.D;
        return self;
    }
}

#[derive(Default)]
struct Average {
    accuracy: f64,
    percision: f64,
    recall: f64,
    count: f64,
}

impl Calculate for Average {
    fn accuracy(&self) -> f64 {
        self.accuracy / self.count
    }

    fn percision(&self) -> f64 {
        self.percision / self.count
    }

    fn recall(&self) -> f64 {
        self.recall / self.count
    }
}

impl From<&Metric> for Average {
    fn from(metric: &Metric) -> Self {
        Self {
            accuracy: metric.accuracy(),
            percision: metric.percision(),
            recall: metric.recall(),
            count: 1f64,
        }
    }
}

impl std::ops::Add<Average> for Average {
    type Output = Average;
    fn add(mut self, rhs: Self) -> Self {
        self.accuracy += rhs.accuracy;
        self.percision += rhs.percision;
        self.recall += rhs.recall;
        self.count += rhs.count;
        return self;
    }
}

impl Metric {
    pub fn save(metrics: &[Metric], file_path: &String) -> Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        let (avg, global_metric) = metrics.into_iter().try_fold(
            (Average::default(), Metric::default()),
            |(avg, global), x| -> Result<_> {
                writeln!(&mut f, "{} {}", x, x as &dyn Calculate)?;
                return Result::Ok((avg + x.into(), global + x));
            },
        )?;
        writeln!(&mut f, "{}", &global_metric as &dyn Calculate)?;
        writeln!(&mut f, "{}", &avg as &dyn Calculate)?;
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
}
