extern crate rand;
use rand::{thread_rng, Rng};

#[path = "../src/lib.rs"]
mod lib;

fn main() {
    let mut a: Vec<f64> = vec![];
    let mut b: Vec<f64> = vec![];

    let mut rng = thread_rng();

    for i in 0..1000 {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        a.push(x);
        b.push(y);
    }

    lib::plot(&(a, b));
}
