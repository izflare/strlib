extern crate strlib;
// extern crate bit_vec;

use strlib::rlenc;
// use bit_vec::BitVec;

fn main() {
    let v: Vec<u32> = vec![1,1,1,1,2,2,2,3,3,1,1,3,3,3,3,3,3,2,2,2,2,1];
    let mut s: Vec<u32> = Vec::new();
    let mut l: Vec<u32> = Vec::new();
    rlenc::encode(&v, &mut s, &mut l);
    let mut u: Vec<u32> = Vec::new();
    rlenc::decode(&s, &l, &mut u);
    println!("v: {:?}",v);
    println!("s: {:?}", s);
    println!("l: {:?}", l);
    println!("u: {:?}", u);
}
