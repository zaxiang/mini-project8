use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_string>", args[0]);
        std::process::exit(1);
    }

    let input_data = &args[1];
    let processed_data = process_data(input_data);

    println!("Processed data: {}", processed_data);
}

fn process_data(data: &str) -> String {
    data.to_uppercase().chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        assert_eq!(process_data("rust"), "RUST");
        assert_eq!(process_data("hello"), "HELLO");
        assert_eq!(process_data("world"), "WORLD");
        assert_eq!(process_data("this is a test"), "THIS IS A TEST");
    }
}
