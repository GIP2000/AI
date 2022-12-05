use neural_network::parse::user_input::get_input;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_distr::{Distribution, Normal, Uniform};
use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    let train_file: String = get_input("output train file name: ");
    let test_file: String = get_input("output test file name: ");
    // let num_points: u32 = get_input("How many points per spiral");
    let num_points = 3000;
    let mut rng = thread_rng();
    let uniform: Uniform<f64> = Uniform::new(1.0, 15.0);
    let normal: Normal<f64> = Normal::new(0.0, 0.02).expect("Error making Normal dist");

    let r_1: Vec<_> = (0..num_points).map(|_| uniform.sample(&mut rng)).collect();
    let r_2: Vec<_> = (0..num_points).map(|_| uniform.sample(&mut rng)).collect();

    let x_1: Vec<_> = r_1
        .iter()
        .map(|r| r * r.cos() + normal.sample(&mut rng))
        .collect();
    let y_1: Vec<_> = r_1
        .iter()
        .map(|r| r * r.sin() + normal.sample(&mut rng))
        .collect();

    let x_2: Vec<_> = r_2
        .iter()
        .map(|r| -r * r.cos() + normal.sample(&mut rng))
        .collect();
    let y_2: Vec<_> = r_2
        .iter()
        .map(|r| -r * r.sin() + normal.sample(&mut rng))
        .collect();

    // 80% training 20% testing data

    let mut indexes = (0..(num_points * 2)).collect::<Vec<_>>();
    indexes.shuffle(&mut rng);

    let train_size = (2f64 * num_points as f64 * 0.8).ceil() as usize;

    let mut train_f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(train_file)
        .expect("Couldn't make train file");
    writeln!(&mut train_f, "{} 2 1", train_size).expect("Error writting to file");

    let mut test_f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(test_file)
        .expect("Couldn't make test file");
    writeln!(&mut test_f, "{} 2 1", 2 * num_points - train_size).expect("Error writting to file");

    for (i, &loc) in indexes.iter().enumerate() {
        let f = match i >= train_size {
            true => &mut test_f,
            false => &mut train_f,
        };
        let (x, y, loc, class) = match loc >= num_points {
            true => (&x_2, &y_2, loc - num_points, 1),
            false => (&x_1, &y_1, loc, 0),
        };
        writeln!(f, "{} {} {}", x[loc], y[loc], class).expect("failed to write to file");
    }
}
