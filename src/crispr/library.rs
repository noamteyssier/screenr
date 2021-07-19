use std::{collections::HashMap, fs::File, io::Write};
use regex::Regex;
use crate::reader::{FastqRead, FastqRecord};
use super::{Fasta, utils::reverse_complement};

pub struct Library {
    lib: HashMap<String, String>,
    counts: HashMap<String, Vec<u32>>,
    genes: HashMap<String, String>,
    fwd_regex: Regex,
    rev_regex: Regex,
    num_fwd: u32,
    num_rev: u32,
    n_samples: usize
}
impl Library {

    /// Initializes an empty library
    pub fn new(guide_seq: &str, n_samples: usize) -> Self {
        let rc_guide = reverse_complement(guide_seq);
        let fwd_regex = Self::build_regex(guide_seq);
        let rev_regex = Self::build_regex(&rc_guide);
        Self {
            lib: HashMap::new(),
            counts: HashMap::new(),
            genes: HashMap::new(),
            fwd_regex, rev_regex,
            num_fwd: 0,
            num_rev: 0,
            n_samples
        } 
    }

    /// Creates the regex expression to match the guide 
    fn build_regex(seq: &str) -> Regex {
        let expr = format!(r"{}", seq);
        Regex::new(&expr)
            .expect("Error: Could not create regex from seq")
    }

    /// Parses gene information from sequence header
    fn parse_gene(&self, name: &str) -> String {
        name.split("_").next().unwrap().to_string()
    }

    /// Reads in a FASTA formatted file and initializes library
    pub fn load_library(&mut self, filename: &str) {
        let fr = Fasta::new(filename)
            .expect("Error: Library could not be found");
        for record in fr.into_iter() {
           
            // sequence -> name mapping
            self.lib.insert(
                record.get_seq().to_string(),
                record.get_name().to_string(), 
            );

            // name -> counts mapping 
            self.counts.insert(
                record.get_name().to_string(),
                vec![0; self.n_samples]
            );

            // name -> gene mapping
            self.genes.insert(
                record.get_name().to_string(), 
                self.parse_gene(record.get_name())
            );
        }
    }

    /// Truncate the sequence to the 19bp protospacer
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
    fn match_lib(&mut self, seq: &str, idx: usize) {
        if self.lib.contains_key(seq) {
            let name = self.lib.get(seq).unwrap();
            self.counts.get_mut(name).unwrap()[idx] += 1;
        }
    } 

    /// Matches the sequence against the library
    pub fn match_seq(&mut self, record: &FastqRecord, idx: usize) {
        match self.get_direction(record) {
            Some(seq) => self.match_lib(&seq, idx),
            _ => ()
        };
    }

    /// Prints the count table to stdout
    pub fn print_count_table(&self, labels: Vec<&str>) {
        print!("sgRNA\tGene");
        for l in labels {
            print!("\t{}", l);
        }
        print!("\n");

        self.counts
            .keys()
            .for_each(|k| {
                let gene = self.genes.get(k).unwrap();
                let counts = self.counts.get(k).unwrap();

                print!("{}\t{}", k, gene);
                for c in counts.iter() {
                    print!("\t{}", c);
                }
                print!("\n");
            });
    }

    /// Writes the count table to file
    pub fn write_count_table(&mut self, filename: &str, labels: Vec<&str>) {

        // open file
        let mut file = File::create(filename)
            .expect("Unable to create file");
        

        // write header
        file.write_all("sgRNA\tGene".as_bytes());
        for l in labels {
            file.write_all(format!("\t{}", l).as_bytes());
        }
        file.write_all("\n".as_bytes());

        // write counts
        self.counts
            .keys()
            .for_each(|k| {
                let gene = self.genes.get(k).unwrap();
                let counts = self.counts.get(k).unwrap();

                file.write_all(format!("{}\t{}", k, gene).as_bytes());
                for c in counts.iter() {
                    file.write_all(format!("\t{}", c).as_bytes());
                }
                file.write_all("\n".as_bytes());
            });

    }

    /// Summary statistics on forward/reverse/total reads
    pub fn summary(&self) {
        println!(">>Fwd Reads:\t{}", self.num_fwd);
        println!(">>Rev Reads:\t{}", self.num_rev);
        println!(">>Total Reads:\t{}", self.num_fwd + self.num_rev);
    }

    fn clear_summary(&mut self) {
        self.num_fwd = 0;
        self.num_rev = 0;
    }

    /// Match all sequences in a given reader
    pub fn match_reader<R: FastqRead + Iterator<Item = FastqRecord>>(&mut self, reader: &mut R, idx: usize) {
        reader
            .into_iter()
            .for_each(|x| self.match_seq(&x, idx));

        self.summary();
        self.clear_summary();
    }
}
