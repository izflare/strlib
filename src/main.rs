extern crate strlib;
extern crate bit_vec;

// use strlib::mtf;
use strlib::delta;
use bit_vec::BitVec;

fn main() {
    let v: Vec<u32> = vec![1, 2, 5, 3, 4];
    let mut bv: BitVec = BitVec::new();
    delta::encode(&v, &mut bv);
    println!("v: {:?}", v);
    println!("bv: {:?}", bv);
    let mut w: Vec<u32> = Vec::new();
    delta::decode(&bv, &mut w);
    println!("w: {:?}", w);
}
