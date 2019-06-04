extern crate strlib;
extern crate bit_vec;

use strlib::mtf;
// use strlib::delta;
// use bit_vec::BitVec;

fn main() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 2, 4, 9, 7, 10, 8, 11, 7, 3, 12, 4, 11, 1, 13, 14];
    let mut w: Vec<usize> = Vec::new();
    let mut z: Vec<u32> = Vec::new();
    mtf::convert(&v, &mut w, &mut z);
    println!("v: {:?}", v);
    println!("z: {:?}", z);
    println!("w: {:?}", w);
    let mut u: Vec<u32> = Vec::new();
    mtf::restore(&w, &z, &mut u);
    println!("u: {:?}", u);
}
