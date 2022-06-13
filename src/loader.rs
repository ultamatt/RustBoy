use std::io;
use std::path::Path;
use std::io::{ Read, BufReader };
use std::fs::File;

pub struct Loader {
    filename: String,
    filecontents: Vec <u8>
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            filename:String::from("new loader"),
            filecontents: vec![]
        }
    }

    pub fn load(&mut self) -> i32 {
        println!("Enter the file you would like to load");

        let mut filename = String::new();
        match io::stdin().read_line(&mut filename) {
            Ok(_n) => {
                //println!("{n} bytes read");
                //println!("\"{filename}\"");
            }
            Err(error) => {
                println!("error: {error}");
                return -1;
            },
        }

        let len = filename.len();
        filename.truncate(len - 1);
        println!("\"{filename}\"");

        // Create a path to the desired file
        let path = Path::new(&filename);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        // Read file into vector.
        reader.read_to_end(&mut buffer);

        // Read.
        // for value in buffer {
        //     println!("BYTE: {}", value);
        // };
        self.filename=filename;
        self.filecontents=buffer;

        return 0;
    }

    pub fn filename(&self) -> &String {
        &self.filename
    }

    pub fn filecontents(&self) -> &Vec<u8> {
        &self.filecontents
    }

    pub fn show_debug(&self) -> () {
        println!("filename -> {}", &self.filename);
        println!("filecontents -> {:#?}", &self.filecontents);
    }
}
