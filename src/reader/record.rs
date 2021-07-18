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
            & self.seq.is_empty() 
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
            & self.seq.is_empty() 
            & self.qual.is_empty()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_seq(&self) -> &str {
        &self.seq
    }
    pub fn get_qual(&self) -> &str {
        &self.qual
    }
}
