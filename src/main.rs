extern crate strlib;
extern crate bit_vec;

use strlib::fixed;
use bit_vec::BitVec;

fn main() {

    let v: Vec<u32> = vec![1,2,32,3,2,2,1,33,1,13,1,5,4,1,2,431,43,1,43,0,0,0,0,0,0,0,0,0,0,0];
    println!("v: {:?}",v);

    let mut bv: BitVec = BitVec::new();
    fixed::encode(&v, &mut bv);
    println!("bv: {:?}", bv);

    let mut u: Vec<u32> = Vec::new();
    fixed::decode(&bv, &mut u);
    println!("u: {:?}", u);

    assert_eq!(v, u);
}
