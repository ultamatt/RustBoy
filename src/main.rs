// RUST BOY
mod loader;
mod repl;

use loader::Loader;
use repl::Repl;
use repl::ReplInteraction;

use std::env;

fn main() {
    // For debugging
    env::set_var("RUST_BACKTRACE", "1");

    // Get our loader to load up the rom for us
    let our_loader = Loader::new();
    let loader_interaction = ReplInteraction {
        invocation: String::from("load"),
        function: our_loader.load
    };

    let our_repl = Repl {
        interactions: vec![loader_interaction]
    };

    our_repl.prompt();
}
