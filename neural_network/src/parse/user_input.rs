use std::{io::stdin, str::FromStr};

pub fn get_input<T: FromStr>(prompt: &str) -> T {
    println!("{}", prompt);
    let mut path: String = "".to_string();
    match stdin().read_line(&mut path) {
        Result::Ok(_) => {
            if let Some('\n') = path.chars().next_back() {
                path.pop();
            }
            if let Some('\r') = path.chars().next_back() {
                path.pop();
            }

            match path.parse::<T>() {
                Result::Ok(v) => v,
                Result::Err(_) => {
                    println!("Entered value not parsable, try again");
                    return get_input(prompt);
                }
            }
        }
        Result::Err(_) => {
            println!("Error Invalid reading file try again");
            return get_input(prompt);
        }
    }
}
