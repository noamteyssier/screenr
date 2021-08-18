mod reader;
mod crispr;

use crispr::{Library, assign_reader, ReaderType};
use reader::{Fastq, FastqGz};

use std::collections::HashSet;
use std::path::Path;
use clap::{App, Arg};

fn get_args() -> App<'static, 'static> {
    App::new("Screenr")
        .version("0.3")
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
            .help("Sets the input fasta file to use as a guide library [can also provide 'all' or 'h[1..7]' without path]")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .help("Sets the output tsv to write guide counts to (default = stdout)")
            .takes_value(true))
        .arg(Arg::with_name("NAMES")
            .short("n")
            .long("names")
            .help("Sets the sample name for file(s)")
            .required(true)
            .min_values(1))
        .arg(Arg::with_name("GUIDE")
            .short("g")
            .long("guide")
            .help("Sets the input guide sequence to match on")
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
fn validate_inputs(input_sequences: &Vec<&str>, library_filename: &str, names: &Vec<&str>, guide_sequence: &str) {
   
    // validates `input_sequences` and `names` are equal lengths
    assert_eq!(
        input_sequences.len(),
        names.len(),
        "Number of files + number of names provided are unequal"
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

/// creates the known libraries 
fn build_known_library() -> HashSet<String> {

    // builds known libraries
    let mut known_libraries = HashSet::new();
    
    known_libraries.insert(
        "all".to_string(),
        );
    
    for i in 1..8 {
        let basename = format!("h{}", i);
        known_libraries.insert(
            basename.to_string(),
            );
    }

    known_libraries
}

/// If provided library is known and supported will reassign shorthand to fixed path
fn reassign_library(lib: &str) -> Option<String> {
    let cargo_path = env!("CARGO_MANIFEST_DIR");
    let known_libraries = build_known_library();
    if known_libraries.contains(lib) {
        Some(format!("{}/data/libraries/CRISPRi_v2_crop28.{}.fasta.gz", cargo_path, lib))
    }
    else {
        None
    }
}

fn main() {
    let matches = get_args().get_matches();
    
    let input_sequences: Vec<&str> = matches.values_of("INPUT")
        .expect("ERROR: unable to load provided input")
        .collect();
    let mut library_filename = matches.value_of("LIBRARY")
        .expect("ERROR: unable to load provided library").to_string();
    let output_filename = matches.value_of("OUTPUT");
    let names: Vec<&str> = matches.values_of("NAMES")
        .expect("ERROR: unable to load provided label")
        .collect();
    let guide_sequence = matches.value_of("GUIDE")
        .expect("ERROR: unable to load provided guide");
    

    match reassign_library(&library_filename) {
        Some(pathname) => {
            library_filename = pathname;
        },
        None => {}
    };

    // validate inputs
    validate_inputs(&input_sequences, &library_filename, &names, guide_sequence);

    // load library
    let mut library = Library::new(guide_sequence, input_sequences.len());
    library.load_library(&library_filename).expect("ERROR: Could not load library");

    // iterate sequences
    for idx in 0..input_sequences.len() {
        run_matching(
            input_sequences[idx], 
            &mut library,
            idx);
    }

    // write output
    match output_filename {
        Some(ofn) => {
            library.write_count_table(ofn, names)
                .expect("ERROR: Could not write count table");
        },
        None => {
            library.print_count_table(names);
        }
    }
}

