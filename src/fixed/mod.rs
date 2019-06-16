extern crate bit_vec;

use bit_vec::BitVec;

pub fn to_bv(x: u32, length: u32, bv: &mut BitVec) -> () {
    let mut z = x;
    z = z.rotate_right(length);
    for _ in 0..length {
        z = z.rotate_left(1);
        bv.push(z % 2 == 1);
    }
}

pub fn encode(v: &Vec<u32>, bv: &mut BitVec) -> () {
    let mut m: u32 = 0;
    for e in v {if *e > m {m = *e;}}
    let r = 32 - m.leading_zeros();
    to_bv(r, 32, bv);
    for e in v {
        to_bv(*e, r, bv);
    }
}

pub fn decode(bv: &BitVec, v: &mut Vec<u32>) -> () {

    let mut r = 0;
    let mut u: u32 = 0;
    for i in 0..bv.len() {
        if i < 32 {r <<= 1; if bv[i] {r += 1;}}
        else {
            u <<= 1; if bv[i] {u += 1;}
            if (i - 32) % r == (r - 1) {v.push(u); u = 0;}
        }
    }

}

