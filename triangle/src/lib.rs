pub struct Triangle{
    a:u64,
    b:u64,
    c:u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        //unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
        if (sides == [0, 0, 0]) || (sides[0] > sides[1] + sides[2]) || (sides[1] > sides[0] + sides[2]) || (sides[2] > sides[0] + sides[1]) {
            None
        }
        else {
            let triangle = Triangle{a:sides[0], b:sides[1], c:sides[2]};
            Some(triangle)
        }
    }

    pub fn is_equilateral(&self) -> bool {
        //unimplemented!("Determine if the Triangle is equilateral.");
        if self.a == self.b && self.b == self.c {
            true
        }
        else {
            false
        }
    }

    pub fn is_scalene(&self) -> bool {
        //unimplemented!("Determine if the Triangle is scalene.");
        if self.a != self.b && self.b != self.c && self.a != self.c {
            true
        }
        else {
            false
        }
    }

    pub fn is_isosceles(&self) -> bool {
        //unimplemented!("Determine if the Triangle is isosceles.");
        if (self.a == self.b && self.b != self.c) || (self.a == self.c && self.c != self.b) || (self.b == self.c && self.c != self.a) {
            true
        }
        else {
            false
        }
    }
}

//Original Problem Link : https://exercism.io/tracks/rust/exercises/triangle/solutions/0ec26e5715db4066a0148374471adccf