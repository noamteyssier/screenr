use std::io::Error;
use super::{
    FastqRecord, FastaRecord
};

pub trait FastaRead {

    fn pop_line(&mut self) -> Result<bool, Error>;

    fn next_record(&mut self) -> Option<FastaRecord>;

}

pub trait FastqRead {

    fn pop_line(&mut self) -> Result<bool, Error>;

    fn next_record(&mut self) -> Option<FastqRecord>;

}
