// RUST BOY
mod loader;
use loader::Loader;

fn main() {
    println!("Hello, world!");
    let loadTry = Loader::new(String::from("pokemon"));
    println!("You wanted to load up {}", loadTry.file());
}
