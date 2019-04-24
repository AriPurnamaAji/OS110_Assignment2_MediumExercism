#[derive(Debug, PartialEq, Eq)]

pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    //unimplemented!("classify {}", num);
    let mut f:Vec<u64> = Vec::new();
    for i in 1..(num/2)+1 {
        if num % i == 0 {
            f.push(i)
        }
    }

    let mut aliquot_sum:u64 = 0;
    for i in f.iter() {
        aliquot_sum = aliquot_sum + i;
    }
    if num == 0 {
        None
    }
    else if aliquot_sum == num {
        Some(Classification::Perfect)
    }
    else if aliquot_sum > num {
        Some(Classification::Abundant)
    }
    else if aliquot_sum < num {
        Some(Classification::Deficient)
    }
    else {
        None
    }
}

//Original Problem Link : https://exercism.io/tracks/rust/exercises/perfect-numbers/solutions/b68dd705d8a5493a93106b8689615616
//Reference : https://mon26harista.wordpress.com/2012/05/04/perfect-number-bilangan-sempurna/
