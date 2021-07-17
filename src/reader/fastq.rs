use std::{fs::File, io::{BufReader, BufRead, Error}};
use super::{FastqRead, FastqRecord};

pub struct Fastq {
   bufreader: BufReader<File>,
   line: String
}

impl FastqRead for Fastq {

    fn pop_line(&mut self) -> Result<bool, Error> {
        self.line.clear();
        let len = self.bufreader.read_line(&mut self.line)
            .expect("IO error in reading fastq");
        Ok(len > 0)
    }

    fn next_record(&mut self) -> Option<FastqRecord> {
        let mut rec = FastqRecord::new();

        for i in 0..4 {
            if self.pop_line().expect("Unexpected end to file") {
                match i {
                    0 => rec.add_name(self.line.trim()),
                    1 => rec.add_seq(self.line.trim()),
                    3 => rec.add_qual(self.line.trim()),
                    _ => {}
                };
            }
        }
        if !rec.is_empty(){
            Some(rec)
        } else {
            None
        }
    }

}

impl Iterator for Fastq {

   type Item = FastqRecord;

   fn next(&mut self) -> Option<Self::Item> {
        self.next_record()
   }

}

impl Fastq {

    /// Creates a new buffer for a provided file
    pub fn new(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let bufreader = BufReader::new(file);
        let line = String::new();
        let fqr = Self {
            bufreader,
            line
        };
        Ok(fqr)
    }
}


