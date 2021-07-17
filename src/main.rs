mod reader;
mod crispr;
use crispr::Library;
use reader::FastqGz;

fn main() {
    let filename = "data/test.fastq.gz";
    let lib_filename = "data/CRISPRi_v2_crop28.fa";
    let fqr = FastqGz::new(filename)
        .expect("Could not create reader");

    let mut lib = Library::new();
    lib.load_fasta(lib_filename);
}

