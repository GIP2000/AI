use neural_network::network::metric::Metric;
use neural_network::network::Network;
use neural_network::parse::file::{parse_data_file, parse_weight_file};
use neural_network::parse::user_input::get_input;

fn main() {
    let weight_file_name = get_input("Please Enter a file path for the weights: ");
    let (shape, weights) = parse_weight_file(&weight_file_name).expect("File Shape Invalid");
    let mut nn = Network::new(shape, weights);
    println!("{:?}", nn);

    let testing_file_path = get_input("Please Enter a file path for the testing data: ");
    let (X, Y) = parse_data_file::<u8>(&testing_file_path).expect("error parsing training file");

    let results = nn.test(X, Y);
    let output_file_path = get_input("Please Enter a file path for the result file: ");
    Metric::save(results, &output_file_path).expect("Error writting result file");
}
