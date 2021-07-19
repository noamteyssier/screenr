use super::Library;
use crate::reader::{FastqRead, FastqRecord};


pub struct Matcher <B> {
    reader: B,
    library: Library
}
impl <B> Matcher <B> 
where
    B: FastqRead + Iterator<Item = FastqRecord>
{
    pub fn new(reader: B, library: Library) -> Self {
        Self {
            reader, library
        }
    }

    pub fn run(&mut self) {
        self.library.match_reader(&mut self.reader);
    }

    pub fn summary(&mut self, output_filename: &str) {
        self.library.write_count_table(output_filename);
        self.library.summary();
    }
}
