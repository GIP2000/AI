use neural_network::network::Network;
use neural_network::parse::file::{parse_data_file, parse_weight_file};
use neural_network::parse::user_input::get_input;

fn main() {
    let weight_file_name = get_input("Please Enter a file path for the weights: ");
    let (shape, weights) = parse_weight_file(&weight_file_name).expect("File Shape Invalid");
    let mut nn = Network::new(shape, weights);
    println!("{:?}", nn);
    let training_file_path = get_input("Please Enter a file path for the training data: ");
    let (X, Y) = parse_data_file::<f64>(&training_file_path).expect("error parsing training file");
    let epoch: u32 = get_input("What is the epoch value?");
    let learning_rate: f64 = get_input("What is the learning rate?");
    nn.train(X, Y, epoch, learning_rate);

    nn.save(&get_input("What is the output file"))
        .expect("Error Saving the file");

    println!("File Saved Succesfully!");
}
