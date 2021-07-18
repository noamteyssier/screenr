mod reader;
mod crispr;
use clap::{App, Arg};
use crispr::Library;
use reader::{Fastq, FastqGz, FastqRead, FastqRecord};


enum FQR {
    Fq, FqGz
}

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

fn run_matching<R: FastqRead + Iterator<Item = FastqRecord>>(fqr: R, ofn: &str, lib_fn: &str, guide_seq: &str) {
    let mut lib = Library::new(guide_seq);
    lib.load_library(lib_fn);

    for rec in fqr.into_iter() {
        lib.match_seq(&rec);
    }

    lib.write_count_table(ofn);
    lib.summary();
}

fn assign_fqr(input_filename: &str) -> Option<FQR> {
    if input_filename.contains(".fastq.gz") {
        Some(FQR::FqGz)
    }
    else if input_filename.contains(".fq.gz") {
        Some(FQR::FqGz)
    }
    else if input_filename.contains(".fastq") {
        Some(FQR::Fq)
    }
    else if input_filename.contains(".fastq") {
        Some(FQR::Fq)
    }
    else {
        None
    }
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

    match assign_fqr(input_sequences) {
        Some(FQR::FqGz) => {
            run_matching(
                FastqGz::new(input_sequences).expect(" "),
                output_filename,
                library_filename,
                guide_sequence
                );
        },
        Some(FQR::Fq) => {
            run_matching(
                Fastq::new(input_sequences).expect(" "),
                output_filename,
                library_filename,
                guide_sequence
                );
        },
        _ => {}
    }

}

