use std::collections::HashMap;
use super::Fasta;

pub struct Library {
    lib: HashMap<String, String>
}
impl Library {
    pub fn new() -> Self {
       Self {
            lib: HashMap::new()
       } 
    }
    pub fn load_fasta(&mut self, filename: &str) {
        let fr = Fasta::new(filename)
            .expect("Error: Library could not be found");
        for record in fr.into_iter() {
            println!("{:?}", record);
        }
    }
}
