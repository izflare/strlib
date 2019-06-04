extern crate strlib;
extern crate bit_vec;

use strlib::mtf;
// use strlib::delta;
// use bit_vec::BitVec;

fn main() {
    let v: Vec<u32> = vec![1,4,5,1,6,7,4,1,13,9,16,9,20,4,10];
    let mut w: Vec<usize> = Vec::new();
    let mut z: Vec<u32> = Vec::new();
    mtf::convert(&v, &mut w, &mut z);
    println!("v: {:?}", v);
    println!("z: {:?}", z);
    println!("w: {:?}", w);
}
