extern crate strlib;
extern crate bit_vec;

use strlib::block_fixed;
use bit_vec::BitVec;

fn main() {
    let v: Vec<u32> = vec![32,4,3,2,1,1,  1,2,3,4,5,6, 1,1,1,1,1,1, 2,3];
    let mut bv: BitVec = BitVec::new();

    block_fixed::encode(&v, 6, &mut bv);
    let mut u: Vec<u32> = Vec::new();
    block_fixed::decode(&bv, &mut u);
    println!("v: {:?}",v);
    println!("bv: {:?}", bv);
    println!("u: {:?}", u);
}
