#[derive(Debug)]
pub struct FastaRecord {
    name: String,
    seq: String
}
impl FastaRecord {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            seq: String::new()
        }
    }
    pub fn add_name(&mut self, s: &str) {
        self.name.push_str(s) 
    }
    pub fn add_seq(&mut self, s: &str) {
        self.seq.push_str(s) 
    }
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() 
            | self.seq.is_empty() 
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_seq(&self) -> &str {
        &self.seq
    }
}
#[derive(Debug)]
pub struct FastqRecord {
    name: String,
    seq: String,
    qual: String
}
impl FastqRecord {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            seq: String::new(),
            qual: String::new()
        }
    }
    pub fn add_name(&mut self, s: &str) {
        self.name.push_str(s) 
    }
    pub fn add_seq(&mut self, s: &str) {
        self.seq.push_str(s) 
    }
    pub fn add_qual(&mut self, s: &str) {
        self.qual.push_str(s) 
    }
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() 
            | self.seq.is_empty() 
            | self.qual.is_empty()
    }
    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_seq(&self) -> &str {
        &self.seq
    }
    #[allow(dead_code)]
    pub fn get_qual(&self) -> &str {
        &self.qual
    }
}

#[test]
fn test_record_fasta() {
    let name = "hello";
    let seq = "ACTGAA";

    let mut rec = FastaRecord::new();
    assert!(rec.is_empty(), "No fields present");
    rec.add_name(name);
    assert!(rec.is_empty(), "Only Name present");
    rec.add_seq(seq);
    assert!(!rec.is_empty(), "Failed empty test");

    assert_eq!(rec.get_name(), name);
    assert_eq!(rec.get_seq(), seq);
}

#[test]
fn test_record() {
    let name = "hello";
    let seq = "ACTGAA";
    let qual = "Qt2135";

    let mut rec = FastqRecord::new();
    assert!(rec.is_empty(), "No fields present");
    rec.add_name(name);
    assert!(rec.is_empty(), "Only Name present");
    rec.add_seq(seq);
    assert!(rec.is_empty(), "Only Name + Seq present");
    rec.add_qual(qual);
    assert!(!rec.is_empty(), "Failed empty test");

    assert_eq!(rec.get_name(), name);
    assert_eq!(rec.get_seq(), seq);
    assert_eq!(rec.get_qual(), qual);
}
