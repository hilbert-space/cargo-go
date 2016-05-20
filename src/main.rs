use std::process;

mod open;

fn main() {
    let path = "https://crates.io";
    if let Err(_) = open::open(path) {
        println!("Failed to open {:?}.", path);
        process::exit(1);
    }
}
