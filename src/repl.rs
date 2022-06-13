use std::io;

// Repl helper,
// Idea here is the have each 'module' provide a string match with args
//  and those execute

// One single repl interaction, defined by the 'invocation' string
pub struct ReplInteraction {
    invocation: String,
    function: &fn(String, Vec<String>) -> i32
}

pub struct Repl {
    interactions: Vec <ReplInteraction>
}

impl Repl {
    pub fn prompt(&self) -> i32 {
        print!(":> ");

        let mut wholestring = String::new();
        match io::stdin().read_line(&mut wholestring) {
            Ok(n) => {
                let len = wholestring.len();
                wholestring.truncate(len - 1);

                let mut splitstring = wholestring.split(" ");
                let mut vec = splitstring.collect::<Vec<&str>>();
                let first_command = vec.remove(0);
                let command_vector: Vec<String> = vec.into_iter().map(|i| i.to_string()).collect::<Vec<String>>();

                //Match interactions based on command
                // Command {
                //     command: first_command.to_string(),
                //     command_args: command_vector
                // }
                return 0;
            }
            Err(error) => {
                println!("error: {error}");
                // Command {
                //     command: String::from("Unkonwn"),
                //     command_args: vec![]
                // }
                return -1;
            },
        }
    }

    // pub fn display(&self) -> () {
    //     println!("Command is {} and args are {:?}", &self.command, &self.command_args)
    // }
}
