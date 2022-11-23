use neural_network::network::Network;
use neural_network::parse::user_input::get_input;
use neural_network::parse::util::delim_parse;

fn main() {
    let shape: Vec<usize> =
        delim_parse(get_input::<String>("What is the shape of the nn: ").split(' '))
            .expect("Error: Invalid Shape");

    let nn = Network::random_new(shape);

    nn.save(&get_input::<String>(
        "What is the name of the output file: ",
    ))
    .expect("Error Saving to file");
}
