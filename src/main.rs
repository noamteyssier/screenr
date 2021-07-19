mod reader;
mod crispr;
use std::path::Path;

use clap::{App, Arg};
use crispr::{Library, assign_reader, ReaderType};
use reader::{Fastq, FastqGz};


fn get_args() -> App<'static, 'static> {
    App::new("Screenr")
        .version("0.2")
        .author("Noam Teyssier <Noam.Teyssier@ucsf.edu>")
        .about("Parses a provided fastq file for a required guide then matches sequences into a provided sgRNA library to determine sgRNA counts")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .help("Sets the input fastq(s) file to use (*.fastq, *.fq, *.fastq.gz, *.fq.gz)")
            .required(true)
            .min_values(1))
        .arg(Arg::with_name("LIBRARY")
            .short("l")
            .long("library")
            .help("Sets the input fasta file to use as a guide library")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .help("Sets the output tsv to write guide counts to")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("LABEL")
            .short("n")
            .long("name")
            .help("Sets the sample name for file(s)")
            .required(true)
            .min_values(1))
        .arg(Arg::with_name("GUIDE")
            .short("g")
            .long("guide")
            .help("Sets the input guide sequence to match on (default='GTTTAAGAG')")
            .required(false)
            .takes_value(true)
            .default_value("GTTTAAGAG"))
}

/// Performs the matching algorithm
fn run_matching(input_sequences: &str, library: &mut Library, idx: usize) {
    match assign_reader(input_sequences) {
        Some(ReaderType::FASTQ) => {
            let mut reader = Fastq::new(input_sequences).unwrap();
            library.match_reader(&mut reader, idx);
        },
        Some(ReaderType::FASTQGZ) => {
            let mut reader = FastqGz::new(input_sequences).unwrap();
            library.match_reader(&mut reader, idx);
        },
        _ => {}
    };
}

/// Confirms that inputs are in the expected format
fn validate_inputs(input_sequences: &Vec<&str>, library_filename: &str, labels: &Vec<&str>, guide_sequence: &str) {
   
    // validates `input_sequences` and `labels` are equal lengths
    assert_eq!(
        input_sequences.len(),
        labels.len(),
        "Number of files + number of labels provided are unequal"
    );

    // validates `library_filename` exists
    assert!(
        Path::new(library_filename).exists(),
        "Provided library path does not exist"
    );

    // validates `guide_sequence` length is > 5
    assert!(
        guide_sequence.len() > 5,
        "Provided guide sequence must be at least 5 basepairs"
    );
}

fn main() {
    let matches = get_args().get_matches();
    
    let input_sequences: Vec<&str> = matches.values_of("INPUT")
        .expect("ERROR: unable to load provided input")
        .collect();
    let library_filename = matches.value_of("LIBRARY")
        .expect("ERROR: unable to load provided library");
    let output_filename = matches.value_of("OUTPUT")
        .expect("ERROR: unable to load provided output");
    let labels: Vec<&str> = matches.values_of("LABEL")
        .expect("ERROR: unable to load provided label")
        .collect();
    let guide_sequence = matches.value_of("GUIDE")
        .expect("ERROR: unable to load provided guide");
    
    // validate inputs
    validate_inputs(&input_sequences, library_filename, &labels, guide_sequence);

    // load library
    let mut library = Library::new(guide_sequence, input_sequences.len());
    library.load_library(library_filename).expect("ERROR: Could not load library");

    // iterate sequences
    for idx in 0..input_sequences.len() {
        run_matching(
            input_sequences[idx], 
            &mut library,
            idx);
    }

    // write output
    library.write_count_table(output_filename, labels)
        .expect("ERROR: Could not write count table");
}

