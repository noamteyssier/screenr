use std::{fs::File, io::{BufRead, BufReader, Error}};
use flate2::read::MultiGzDecoder;

use super::{FastaRead, FastaRecord};

#[derive(Debug)]
pub struct FastaGz {
    bufreader: BufReader<MultiGzDecoder<File>>,
    line: String
}

impl FastaRead for FastaGz {

    fn pop_line(&mut self) -> Result<bool, Error> {
        self.line.clear();
        let len = self.bufreader.read_line(&mut self.line)
            .expect("IO error in reading fastq");
        Ok(len > 0)
    }

    fn next_record(&mut self) -> Option<FastaRecord> {
        let mut rec = FastaRecord::new();

        for i in 0..4 {
            if self.pop_line().expect("Unexpected end to file") {
                match i {
                    0 => rec.add_name(self.line.trim().trim_start_matches('>')),
                    1 => rec.add_seq(self.line.trim()),
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

impl Iterator for FastaGz {

   type Item = FastaRecord;

   fn next(&mut self) -> Option<Self::Item> {
        self.next_record()
   }

}

impl FastaGz {

    /// Creates a new buffer for a provided file
    pub fn new(filename: &str) -> Result<Self, Error> {
        let file = File::open(filename)?;
        let gzip_conv = MultiGzDecoder::new(file);
        let bufreader = BufReader::new(gzip_conv);
        let line = String::new();
        let fqr = Self {
            bufreader,
            line
        };
        Ok(fqr)
    }
}


