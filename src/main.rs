// RUST BOY
mod loader;
mod repl;

use loader::Loader;
use repl::Repl;

use std::env;

fn main() {
    // For debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get our loader to load up the rom for us
    // let first_loader = Loader::new();
    // first_loader.show_debug();

    let first_repl = Repl::new();
    first_repl.display();
}
