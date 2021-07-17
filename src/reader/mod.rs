pub mod fastq;
pub mod fasta;
pub mod fasta_gz;
pub mod fastq_gz;
pub mod reader;
pub mod record;

pub use fastq::Fastq;
pub use fasta::Fasta;
pub use fasta_gz::FastaGz;
pub use fastq_gz::FastqGz;
pub use record::{FastqRecord, FastaRecord};
pub use reader::{FastqRead, FastaRead};
