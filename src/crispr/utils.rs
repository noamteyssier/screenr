use regex::Regex;

/// Perform a reverse complement on a given sequence
pub fn reverse_complement(s: &str) -> String {
   s.chars()
       .rev()
       .map(|x| {
           match x {
               'A' => 'T',
               'T' => 'A',
               'G' => 'C',
               'C' => 'G',
               _ => x 
           }})
        .collect()
}

/// Defines the different reader formats
pub enum ReaderType {
    FASTQ, FASTQGZ,
    FASTA, FASTAGZ
}

/// Assigns a filetype to specific reader format
pub fn assign_reader(s: &str) -> Option<ReaderType> {
    let re_fastq_gz = Regex::new(r".fastq.gz$|.fq.gz$").expect("Could not compile regex");
    let re_fasta_gz = Regex::new(r".fasta.gz$|.fa.gz$").expect("Could not compile regex");
    let re_fastq = Regex::new(r".fastq$|.fq$").expect("Could not compile regex");
    let re_fasta = Regex::new(r".fasta$|.fa$").expect("Could not compile regex");

    if re_fastq_gz.is_match(s) {
        Some(ReaderType::FASTQGZ)
    }
    else if re_fasta_gz.is_match(s) {
        Some(ReaderType::FASTAGZ)
    }
    else if re_fastq.is_match(s) {
        Some(ReaderType::FASTQ)
    }
    else if re_fasta.is_match(s) {
        Some(ReaderType::FASTA)
    }
    else {
        None
    }
}
