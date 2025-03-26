use std::{ops::{Add, Mul, Sub}, process::Output};

struct AritmeticnoZaporedje<T> {
    a_0 : T,
    d : T,
    current : u32
}

impl<T: Add<Output = T> + Copy> AritmeticnoZaporedje<T> {
    fn n_th(&self, n: u32) -> T {
        let mut previous = self.a_0;
        for _ in 0..n {
            previous = previous + self.d;
        }
        previous
    }

    fn next(&mut self) -> T {
        let value = self.n_th(self.current); 
        self.current += 1; 
        value
    }

    fn current(&self) -> T {
        return self.n_th(self.current)
    }

    fn sum(&self, n: u32) -> T {
        let mut sum = self.a_0; 
        for i in 1..n { 
            sum = sum + self.n_th(i);
        }
        return sum
    }

    fn vsota(&self, other : AritmeticnoZaporedje<T>) -> AritmeticnoZaporedje<T> {
        AritmeticnoZaporedje::new(self.a_0 + other.a_0, self.d + other.d, 0)
    }
}


impl<T> AritmeticnoZaporedje<T> {

    fn new(a_0 : T, d : T, current : u32) -> AritmeticnoZaporedje<T> {
        return AritmeticnoZaporedje {
            a_0 : a_0,
            d : d,
            current : current
        }
    }
    
    fn reset(&mut self) {
        self.current = 0;
    }
}

impl<T: Copy + Mul<Output = T>> AritmeticnoZaporedje<T> {
    fn n_produkt(&self, other: AritmeticnoZaporedje<T>, i: u32) -> AritmeticnoZaporedje<T> {
        return AritmeticnoZaporedje::new(self.a_0 * other.a_0, self.d * other.d, 0)
    }
}

impl<T: PartialEq> PartialEq for AritmeticnoZaporedje<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a_0 == other.a_0 && self.d == other.d
    }
}


fn main() {
    println!("Hello, world!");
}

//////////////////////////////////////////////////
enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz<T> {
    Konstanta(T),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
}

impl<T> Izraz<T>
where 
    T: Add<Output = T>,
    T: Mul<Output = T>,
    T: Copy,
    T: Sub<Output = T>

{ 
    fn eval(&self) -> T {
        // let mut v = 0;
        match self {
            Izraz::Operacija(a, b, c ) => 
                match b {
                    BinOperacija::Minus => a.eval() - c.eval(),
                    BinOperacija::Plus => a.eval() + c.eval(),
                    BinOperacija::Times => a.eval() * c.eval(),
                },
            Self::Konstanta(x) => (*x).try_into().unwrap(),
        }
    }
}

impl<T: Add<Output = T>> Izraz<T> {

    fn collect(&self) -> u32 {
        let mut count = 0;
        count +=
            match self {
                Izraz::Operacija(a,_ ,c ) => a.collect() + c.collect(),
                Self::Konstanta(_) => 1
            };
        return count
    }
}
impl<T: ToString> Izraz<T> {
    fn izpis(&self) -> String {
        let mut str = String::new();
        match self {
            Izraz::Operacija(a, b, c ) => 
                match b {
                    BinOperacija::Minus => str.push_str(&("(".to_owned() + &a.izpis().to_string() + " - " + &c.izpis().to_string() + ")")),
                    BinOperacija::Plus => str.push_str(&("(".to_owned() + &a.izpis().to_string() + " + " + &c.izpis().to_string() + ")")),
                    BinOperacija::Times =>str.push_str(&("(".to_owned() + &a.izpis().to_string() + " * " + &c.izpis().to_string() + ")")),
                },
            Self::Konstanta(x) => str.push_str(&x.to_string()),
        }
        return str
    }
}

impl<T: ToString> ToString for Izraz<T> {
    fn to_string(&self) -> String {
        return(self.izpis())
    }
}
////////////////////////////////////////////////

struct AritmeticnoZaporedjeAST<T> {
    a_0 : Izraz<T>,
    d : Izraz<T>,
    current : u32
}
use AritmeticnoZaporedjeAST as A;

impl<T: Add<Output = T> + Copy + Mul<Output = T> + Sub<Output = T>> A<T> {
    fn n_th(&self, n: u32) -> T {
        let mut previous = self.a_0;
        for _ in 0..n {
            previous = Izraz::Operacija((Box::new(previous)), (BinOperacija::Plus), (Box::new(self.d)));
        }
        previous
    }

    fn next(&mut self) -> T {
        let value = self.n_th(self.current); 
        self.current += 1; 
        value
    }

    fn current(&self) -> T {
        return self.n_th(self.current)
    }

    fn sum(&self, n: u32) -> T {
        let mut sum = &self.a_0; 
        for i in 1..n { 
            sum = &Izraz::Operacija((Box::new(sum)), (BinOperacija::Plus), (Box::new(self.d)));
        }
        return sum.eval()
    }

    fn vsota(&self, other : A<T>) -> A<T> {
        A::new(self.a_0 + other.a_0, self.d + other.d, 0)
    }
}


impl<T> A<T> {

    fn new(a_0 : T, d : T, current : u32) -> A<T> {
        return A {
            a_0 : Izraz::Konstanta(a_0),
            d : Izraz::Konstanta(d),
            current : current
        }
    }
    
    fn reset(&mut self) {
        self.current = 0;
    }
}

impl<T: Copy + Mul<Output = T>> A<T> {
    fn n_produkt(&self, other: A<T>, i: u32) -> A<T> {
        return A::new(self.a_0 * other.a_0, self.d * other.d, 0)
    }
}

impl<T: PartialEq> PartialEq for A<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a_0 == other.a_0 && self.d == other.d
    }
}


fn main() {
    println!("Hello, world!");
}