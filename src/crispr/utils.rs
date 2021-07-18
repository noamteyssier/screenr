/// Perform a reverse complement on a given sequence
pub fn reverse_complement(s: &str) -> String {
   s.chars()
       .rev()
       .map(|x| {
           match x {
               'A' => 'T',
               'T' => 'A',
               'G' => 'C',
               'C' => 'G',
               _ => x 
           }})
        .collect()
}
