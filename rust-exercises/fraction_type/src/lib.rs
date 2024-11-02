use std::arch::x86_64::_mm256_andnot_pd;
use std::ops;

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction(pub i32, pub i32);

impl Fraction {
    pub fn add(&self, other: Fraction) -> Fraction {
        let (n1, d1) = (self.0, self.1);
        let (n2, d2) = (other.0, other.1);
        if d1 == 0 || d2 == 0 {
            panic!("Cannot divide by zero")
        }
        let (n, d) = (n1 * d2 + n2 * d1, d1 * d2);
        simplify(n, d)
    }

    pub fn sub(&self, other: Fraction) -> Fraction {
        let (n1, d1) = (self.0, self.1);
        let (n2, d2) = (other.0, other.1);
        if d1 == 0 || d2 == 0 {
            panic!("Cannot divide by zero")
        }
        let (n, d) = (n1 * d2 - n2 * d1, d1 * d2);
        simplify(n, d)
    }

    pub fn mul(&self, other: Fraction) -> Fraction {
        let (n1, d1) = (self.0, self.1);
        let (n2, d2) = (other.0, other.1);
        let (n, d) = (n1 * n2, d1 * d2);
        simplify(n, d)
    }

    pub fn divide(&self, other: Fraction) -> Fraction {
        let (n1, d1) = (self.0, self.1);
        let (n2, d2) = (other.0, other.1);
        let (n, d) = (n1 * d2, n2 * d1);
        simplify(n, d)
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;
    fn add(self, other: Fraction) -> Self::Output {
        Fraction::add(&self, other)
    }
    
}

/// Calculate the Highest common factor between 2 numbers
fn hcf(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { hcf(b, a % b) }
}

fn simplify(n: i32, d: i32) -> Fraction {
    let h = hcf(n, d);
    Fraction(n/h, d/h)
}