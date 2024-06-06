///////////////////////////////////////////////////////////////////////////////
// 1. naloga
///////////////////////////////////////////////////////////////////////////////

// 1. a

fn f(c: u32) -> u32 {
    c + c
}
fn main() {
    let a = 10;
    let b = 20;
    let d = f(b) + a;
    println!("{d}");
}

// 1. b

fn f(c: u32) -> u32 {
    println!("{c}");
    c * 10
}
fn g(d: u32) -> (u32, u32) {
    (f(10), d)
}
fn main() {
    let a = 10;
    let b = 20;
    let mut c = 3;
    if a > b {
        c = 10;
    }
    let e = g(c);
    println!("{:?}", e);
}

// 1. c

fn g(a: &String) -> usize {
    a.len()
}
fn f(mut b: String) -> String {
    println!("{}", g(&b));
    b.push('!');
    b
}
fn main() {
    let m = String::from("Abeceda");
    let n = f(m);
    println!("{n}")
}


///////////////////////////////////////////////////////////////////////////////
// 2. naloga
///////////////////////////////////////////////////////////////////////////////

impl<T ____________> Stack<T> {

    fn contains(________ self, elt: ________) ________ {
        // preveri, ali sklad vsebuje dani element
    }
    
    fn elements(________ self) ________ {
        // vrne vektor referenc na elemente sklada
    }
    
    fn push(________ self, elt: ________) ________ {
        // na vrh sklada doda element
    }
    
    fn pop(________ self) ________ {
        // z vrha odstrani element in ga vrne
    }
    
    fn peek(________ self) ________ {
        // na vpogled vrne zgornji element na skladu
    }

}


///////////////////////////////////////////////////////////////////////////////
// 3. naloga
///////////////////////////////////////////////////////////////////////////////

// 3. a

fn main() {
    let a = 0;
    for x in 1..10 {
        a += x;
    }
}

// 3. b

fn main() {
    let trojica = (String::from("1"), String::from("2"), String::from("3"));
    let zadnji = trojica.2;
    println!("{:?}", trojica);
    println!("{zadnji}");
}

// 3. c

fn sestej(a: Option<u32>, b: Option<u32>) -> u32 {
    match (a,b) {
        (Some(a), Some(b)) => a + b,
        (Some(a), None) => a,
        (None, Some(b)) => b
    }
}

// 3. d

fn vecji(b1: &Box<u32>, b2: &Box<u32>) -> &Box<u32> {
    if b1 > b2 {
        return b1;
    } else {
        return b2;
    }
}

// 3. e

fn stevilo_manjsih<T>(v: &Vec<T>, x : &T) -> u32 {
    let c = 0;
    for y in v {
        if x > y {
            c += 1;
        }
    }
    c
}
