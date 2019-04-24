pub fn check(candidate: &str) -> bool {
    //unimplemented!("Is {} an isogram?", candidate);
    let spasi:char = ' ';
    let strip:char = '-';
    let candidate1:String = candidate.to_lowercase();
    for i in 0..candidate1.len() {
        for j in i+1..candidate1.len() {
            if candidate1.chars().nth(i).unwrap() == spasi || candidate1.chars().nth(i).unwrap() == strip {
                break
            }
            if candidate1.chars().nth(i) == candidate1.chars().nth(j) {
                return false
            }
        }
    }
    true
}

//Original Problem Link : https://exercism.io/tracks/rust/exercises/isogram/solutions/51b8150c54a24ef7b6401b2946636bcb
