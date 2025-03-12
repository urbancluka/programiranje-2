#[derive(Debug)]

// struct Par {
//     first : u8,
//     second : u8
// }

// use Par as ap; 

// impl ap {
//     fn new_pair(f : u8, s : u8) -> ap{
//         return ap {
//             first : f,
//             second : s
//         };
//     }

//     fn fst(&self) -> u8 {
//         self.first
//     }
// }

// fn main() {
//     let zaporedje = ap {
//         first : 3, 
//         second : 2
//     };
//     let par2 = ap::new_pair(2, 5);
//     println!("{}", par2.fst());
//     println!("{:?}", par2);

// }


struct AritmeticnoZaporedje {
    a_0 : i32,
    d : i32,
    current : u32
}

impl AritmeticnoZaporedje {

    fn n_th(&self, n: u32) -> i32 {
        let n : i32 = n as i32;
        n * self.d + self.a_0
    }

    fn new(a_0 : i32, d : i32, current : u32) -> AP {
        return AP {
            a_0 : a_0,
            d : d,
            current : current
        }
    }

    fn next(&mut self) -> i32 {
        let value = self.a_0 + (self.current as i32 * self.d) ;
        self.current += 1 ;
        return value
    }
    
    fn reset(&mut self) {
        self.current = 0;
    }

    fn current(&self) -> i32 {
        return self.n_th(self.current)
    }

    fn sum(&self, n : u32) -> i32 {
        let mut sum = 0;
        for i in 0..=n {
            sum += self.n_th(i);
        }
        return sum
    }

    fn vsota(&self, other : AritmeticnoZaporedje) -> AritmeticnoZaporedje {
        AritmeticnoZaporedje::new(self.a_0 + other.a_0, self.d + other.d, 0)
    }

    fn produkt(&self, other : AritmeticnoZaporedje, i : u32) -> i32 {
        self.n_th(i) * other.n_th(i)
    }
}

use AritmeticnoZaporedje as AP;

fn main() {

}

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}


