extern crate strlib;

use strlib::mtf;

fn main() {
    let v: Vec<char> = "geeksforgeeks".chars().collect();
    let mut w: Vec<usize> = Vec::new();
    let mut z: Vec<char> = Vec::new();
    println!("v : {:?}", v);
    mtf::convert(&v, &mut w, &mut z);
    println!("w : {:?}", w);
    println!("z : {:?}", z);
}
