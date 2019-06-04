extern crate strlib;
extern crate bit_vec;

use strlib::mtf;
// use strlib::delta;
// use bit_vec::BitVec;

fn main() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 3, 8, 4, 3, 9, 10, 5, 4, 11, 12, 13, 14, 8, 15, 4, 16, 17];
    let mut w: Vec<usize> = Vec::new();
    let mut z: Vec<u32> = Vec::new();
    mtf::convert(&v, &mut w, &mut z);
    println!("v: {:?}", v);
    println!("z: {:?}", z);
    println!("w: {:?}", w);
}
