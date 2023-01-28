// operator overloading
use std::ops::{Add, AddAssign, Neg};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)] // can either derive PartialEq or implement it manually
#[derive(Debug, Clone, Copy)]
struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> Complex<T> {
    fn new(real: T, imag: T) -> Complex<T> {
        Complex { real, imag }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Complex<T>) {
        self.real += other.real;
        self.imag += other.imag;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;

    fn neg(self) -> Complex<T> {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

// partial eq
impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Complex<T>) -> bool {
        self.real == other.real && self.imag == other.imag
    }
}

// full eq
impl<T> Eq for Complex<T> where T: Eq {}

pub fn traits_op_overload() {
    let c1 = Complex::new(1.2, 2.3);
    let c2 = Complex::new(2.3, 3.6);
    println!("c1 = {:?}", c1);
    println!("c2 = {:?}", c2);
    let c3 = c1 + c2;
    println!("c3 = {:?}", c3);

    let mut c4 = Complex::new(1.2, 2.3);
    c4 += c3;
    println!("c4 = {:?}", c4);

    let c5 = -c4;
    println!("c5 = {:?}", c5);

    println!("c4 == c5 = {}", c5 == c5);
}
