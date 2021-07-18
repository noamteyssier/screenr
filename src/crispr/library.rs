use std::{collections::HashMap, fs::File, io::Write};
use regex::Regex;
use crate::reader::FastqRecord;
use super::{Fasta, utils::reverse_complement};

pub struct Library {
    lib: HashMap<String, String>,
    counts: HashMap<String, u32>,
    fwd_regex: Regex,
    rev_regex: Regex,
    num_fwd: u32,
    num_rev: u32
}
impl Library {

    /// Initializes an empty library
    pub fn new(guide_seq: &str) -> Self {
        let rc_guide = reverse_complement(guide_seq);
        let fwd_regex = Self::build_regex(guide_seq);
        let rev_regex = Self::build_regex(&rc_guide);
        Self {
            lib: HashMap::new(),
            counts: HashMap::new(),
            fwd_regex, rev_regex,
            num_fwd: 0,
            num_rev: 0
        } 
    }

    /// Creates the regex expression to match the guide 
    fn build_regex(seq: &str) -> Regex {
        let expr = format!(r"{}", seq);
        Regex::new(&expr)
            .expect("Error: Could not create regex from seq")
    }

    /// Reads in a FASTA formatted file and initializes library
    pub fn load_library(&mut self, filename: &str) {
        let fr = Fasta::new(filename)
            .expect("Error: Library could not be found");
        for record in fr.into_iter() {
            self.lib.insert(
                record.get_seq().to_string(),
                record.get_name().to_string(), 
            );
            self.counts.insert(
                record.get_name().to_string(),
                0
            );
        }
    }

    /// Trunacate the sequence to the 19bp protospacer
    fn truncate_seq(&self, seq: &str) -> Option<String> {
        match self.fwd_regex.find_at(seq, 20) {
            Some(mat) => {
                let trunc_seq = &seq[mat.start()-19 .. mat.end()];
                Some(trunc_seq.to_string())
            },
            _ => None
        }
    }


    /// Determines directionality and truncates string
    /// if sequence is valid in library
    fn get_direction(&mut self, record: &FastqRecord) -> Option<String> {

        // match against the forward guide
        if self.fwd_regex.is_match(record.get_seq()) {
            let seq = record.get_seq();
            match self.truncate_seq(seq) {
                Some(s) => {
                    self.num_fwd += 1; 
                    Some(s)
                },
                None => None 
            }
        }

        // match against the reverse complement of the guide
        else if self.rev_regex.is_match(record.get_seq()) {
            let seq = reverse_complement(record.get_seq());
            match self.truncate_seq(&seq) {
                Some(s) => {
                    self.num_rev += 1; 
                    Some(s)
                },
                None => None 
            }
        }

        // no match
        else {
            None
        }
    }

    /// Matches the sequence against the library 
    /// and increments the named key
    fn match_lib(&mut self, seq: &str) {
        if self.lib.contains_key(seq) {
            let name = self.lib.get(seq).unwrap();
            *self.counts.get_mut(name).unwrap() += 1;
        }
    } 

    /// Matches the sequence against the library
    pub fn match_seq(&mut self, record: &FastqRecord) {
        match self.get_direction(record) {
            Some(seq) => self.match_lib(&seq),
            _ => ()
        };
    }

    /// Prints the count table to stdout
    pub fn print_count_table(&mut self) {
        self.counts
            .retain(|k, v| {println!("{} : {}", v, k); true});
    }

    /// Writes the count table to file
    pub fn write_count_table(&mut self, filename: &str) {
        let mut file = File::create(filename).expect("Unable to create file");
        self.counts
            .retain(|k, v| {
                file.write_all(format!("{}\t{}\n", v, k).as_bytes())
                    .expect("Error: Could not write to file");
                true
                });
    }

    /// Summary statistics on forward/reverse/total reads
    pub fn summary(&self) {
        println!(">>Fwd Reads:\t{}", self.num_fwd);
        println!(">>Rev Reads:\t{}", self.num_rev);
        println!(">>Total Reads:\t{}", self.num_fwd + self.num_rev);
    }
}
