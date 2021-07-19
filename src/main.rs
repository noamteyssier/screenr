mod reader;
mod crispr;
use clap::{App, Arg};
use crispr::{Library, assign_reader, ReaderType, Matcher};
use reader::{Fastq, FastqGz};


fn get_args() -> App<'static, 'static> {
    App::new("sgRNA Counts")
        .version("0.1")
        .author("Noam Teyssier <Noam.Teyssier@ucsf.edu>")
        .about("Parses a provided fastq file for a required guide then matches sequences into a provided sgRNA library to determine sgRNA counts")
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .help("Sets the input fastq file to use (*.fastq, *.fq, *.fastq.gz, *.fq.gz)")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .help("Sets the output tsv to write guide counts to")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("LIBRARY")
            .short("l")
            .long("library")
            .help("Sets the input fasta file to use as a guide library")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("GUIDE")
            .short("g")
            .long("guide")
            .help("Sets the input guide sequence to match on (default='GTTTAAGAG')")
            .required(false)
            .takes_value(true)
            .default_value("GTTTAAGAG"))
}

fn run_matching(input_sequences: &str, output_filename: &str, library_filename: &str, guide_sequence: &str) {
    let mut lib = Library::new(guide_sequence);
    lib.load_library(library_filename);

    match assign_reader(input_sequences) {

        Some(ReaderType::FASTQ) => {
            let reader = Fastq::new(input_sequences).unwrap();
            let mut matcher = Matcher::new(reader, lib);
            matcher.run();
            matcher.summary(output_filename);
        },

        Some(ReaderType::FASTQGZ) => {
            let reader = FastqGz::new(input_sequences).unwrap();
            let mut matcher = Matcher::new(reader, lib);
            matcher.run();
            matcher.summary(output_filename);
        },

        _ => {}

    };

}


fn main() {
    let matches = get_args().get_matches();
    
    let input_sequences = matches.value_of("INPUT")
        .expect("ERROR: unable to load provided input");
    let output_filename = matches.value_of("OUTPUT")
        .expect("ERROR: unable to load provided output");
    let library_filename = matches.value_of("LIBRARY")
        .expect("ERROR: unable to load provided library");
    let guide_sequence = matches.value_of("GUIDE")
        .expect("ERROR: unable to load provided guide");
    
    run_matching(input_sequences, output_filename, library_filename, guide_sequence)

}

