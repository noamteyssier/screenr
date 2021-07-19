mod reader;
mod crispr;
use clap::{App, Arg};
use crispr::{Library, assign_reader, ReaderType};
use reader::{Fastq, FastqGz};


fn get_args() -> App<'static, 'static> {
    App::new("sgRNA Counts")
        .version("0.1")
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
    

    // requires input sequences and labels to be of equal length
    assert_eq!(input_sequences.len(), labels.len());

    let mut library = Library::new(guide_sequence, input_sequences.len());
    library.load_library(library_filename);

    for idx in 0..input_sequences.len() {
        run_matching(
            input_sequences[idx], 
            &mut library,
            idx);
    }

    library.write_count_table(output_filename, labels)
        .expect("ERROR: Could not write count table");

}

