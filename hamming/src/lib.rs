/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
   // unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
   if s1.len() != s2.len() {
       None
   }
   else {
       if s1 == "" && s2 == "" {
           Some(0)
       }
       else {
           let matching = s1.chars().zip(s2.chars()).filter(|&(s1, s2)| s1 != s2).count();
           Some(matching)
       }
   }
}

//Original Problem Link : https://exercism.io/tracks/rust/exercises/hamming/solutions/37775f1c43b946629268d01f3f6301bb
//Reference : https://stackoverflow.com/questions/29504514/whats-the-best-way-to-compare-2-vectors-or-strings-element-by-element