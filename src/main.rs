// RUST BOY
mod loader;
use loader::Loader;
use std::env;

fn main() {
    // For debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get our loader to load up the rom for us
    let first_loader = Loader::new();
    first_loader.show_debug();
}
