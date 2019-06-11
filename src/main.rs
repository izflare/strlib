extern crate strlib;
extern crate bit_vec;

// use strlib::delta;
// use strlib::gamma;
use strlib::ffenc;
use bit_vec::BitVec;
// use strlib::mtf;

fn main() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 2, 4, 9, 7, 10, 8, 11, 7, 3, 12, 4, 11, 1, 13, 14];
    let mut bv: BitVec = BitVec::new();
    ffenc::encode(&v, &mut bv);
    let mut u: Vec<u32> = Vec::new();
    ffenc::decode(&bv, &mut u);
    println!("v: {:?}",v);
    println!("bv: {:?}", bv);
    println!("u: {:?}", u);
}
