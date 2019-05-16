#![feature(test)]
extern crate test;

pub trait Henlo {
    fn wat(&self) -> u32;
}

impl Henlo for u32 {
    fn wat(&self) -> u32 {
        *self
    }
}

pub struct ContainerGeneric<G: Henlo> {
    pub vec: Vec<G>,
}

pub struct Container {
    pub vec: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    
    use crate::*;

    #[bench]
    fn generic(b: &mut Bencher) {
        let henlo = ContainerGeneric {
            vec: vec![0u32; 1000000],
        };
        b.iter(|| {
            let mut sum = 0;
            for e in henlo.vec.iter() {
                sum += e.wat();
            }
            assert_eq!(sum, 0);
        })
    }

    #[bench]
    fn plain(b: &mut Bencher) {
        let henlo = Container {
            vec: vec![0u32; 1000000],
        };
        b.iter(|| {
            let mut sum = 0;
            for e in henlo.vec.iter() {
                sum += e;
            }
            assert_eq!(sum, 0);
        })
    }
}


