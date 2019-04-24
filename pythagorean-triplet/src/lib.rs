use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    //unimplemented!("Given the sum {}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c", sum);
    let _a:u32 = 1;
    let mut result:HashSet<[u32; 3]> = HashSet::new();

    for a in 1..(sum+1)/3 {
        let _b:u32 = a+1;
        for b in 1..(sum+1)/2 {
            let c:u32 = sum - a - b;
            if a*a + b*b == c*c {
                if a < b && b < c {
                    result.insert([a, b, c]);
                }
            }
        }
    }

    return result;
    
}

//Original Problem Link : https://exercism.io/tracks/rust/exercises/pythagorean-triplet/solutions/38d433317fc340c4b87b45a425f2f8f5
//Reference : https://stackoverflow.com/questions/2817848/find-pythagorean-triplet-for-which-a-b-c-1000
