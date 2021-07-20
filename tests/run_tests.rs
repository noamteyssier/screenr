

use screenr::reader::{
    Fasta, FastaGz,
    Fastq, FastqGz
};
use screenr::crispr::Library;


#[test]
fn test_fasta() {
    let reader = Fasta::new("data/test/test.fasta")
        .expect("Unable to open reader");
    
    let mut n_rec = 0;
    reader
        .into_iter()
        .for_each(|_| n_rec += 1);

    assert_eq!(n_rec, 50);
}

#[test]
fn test_fasta_gz() {
    let reader = FastaGz::new("data/test/test.fasta.gz")
        .expect("Unable to open reader");
    
    let mut n_rec = 0;
    reader
        .into_iter()
        .for_each(|_| n_rec += 1);

    assert_eq!(n_rec, 50);
}

#[test]
fn test_fastq() {
    let reader = Fastq::new("data/test/test.fastq")
        .expect("Unable to open reader");
    
    let mut n_rec = 0;
    reader
        .into_iter()
        .for_each(|_| n_rec += 1);

    assert_eq!(n_rec, 2500);
}

#[test]
fn test_fastq_gz() {
    let reader = FastqGz::new("data/test/test.fastq.gz")
        .expect("Unable to open reader");
    
    let mut n_rec = 0;
    reader
        .into_iter()
        .for_each(|_| n_rec += 1);

    assert_eq!(n_rec, 2500);
}

#[test]
fn test_library() {
    let guide_seq = "GTTTAAGAG"; 
    let mut lib = Library::new(guide_seq, 1);
    let lib_created = lib.load_library("data/test/test.fasta")
        .expect("Library unable to be created")
        .expect("File reader not able to be created");
    assert!(lib_created);

    lib.print_count_table(vec!["lib1"]);
}

#[test]
fn test_single_sample() {
    let guide_seq = "GTTTAAGAG"; 
    let mut reader = FastqGz::new("data/test/test.fastq.gz")
        .expect("unable to create fastqgz reader");
    let mut library = Library::new(guide_seq, 1);
    library.load_library("data/test/test.fasta.gz")
        .expect("unable to load library")
        .expect("unable to load library reader");
    library.match_reader(&mut reader, 0);
    library.summary();
}

#[test]
fn test_multi_sample() {
    let guide_seq = "GTTTAAGAG"; 
    let mut reader = FastqGz::new("data/test/test.fastq.gz")
        .expect("unable to create fastqgz reader");
    let mut library = Library::new(guide_seq, 5);
    library.load_library("data/test/test.fasta.gz")
        .expect("unable to load library")
        .expect("unable to load library reader");
    for i in 0..5 {
        library.match_reader(&mut reader, i);
    }
}
