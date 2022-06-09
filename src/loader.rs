use::std;

pub struct Loader {
    file: String,
}

impl Loader {
    pub fn new(to_load: String) -> Loader {
        println!("Loader will attempt to load: {}", to_load);
        return Loader { file: to_load };
    }

    pub fn file(&self) -> &String {
        &self.file
    }
}
