extern crate strlib;
extern crate bit_vec;

use strlib::huffman_coding;
use bit_vec::BitVec;

fn main() {

    let v: Vec<u32> = vec![0,2,1,0,2,2,0,1,3,0,1,2,0,2,3,4,0];
    println!("v: {:?}",v);

    let mut bv: BitVec = BitVec::new();
    huffman_coding::encode(&v, &mut bv);
    println!("bv: {:?}", bv);

    let mut u: Vec<u32> = Vec::new();
    huffman_coding::decode(&bv, &mut u);
    println!("u: {:?}", u);

    assert_eq!(v, u);
}
