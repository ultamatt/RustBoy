use std::io;

// Repl helper,
// Idea here is the have each 'module' provide a string match with args
//  and those execute
pub struct Repl {
    command: String,
    command_args: Vec <String>
}

impl Repl {
    pub fn new() -> Repl {
        println!("Enter command");

        let mut wholestring = String::new();
        match io::stdin().read_line(&mut wholestring) {
            Ok(n) => {
                let len = wholestring.len();
                wholestring.truncate(len - 1);

                let mut splitstring = wholestring.split(" ");
                let mut vec = splitstring.collect::<Vec<&str>>();
                let first_command = vec.remove(0);
                let command_vector: Vec<String> = vec.into_iter().map(|i| i.to_string()).collect::<Vec<String>>();
                Repl {
                    command: first_command.to_string(),
                    command_args: command_vector
                }
            }
            Err(error) => {
                println!("error: {error}");
                Repl {
                    command: String::from("Unkonwn"),
                    command_args: vec![]
                }
            },
        }
    }

    pub fn display(&self) -> () {
        println!("Command is {} and args are {:?}", &self.command, &self.command_args)
    }
}
