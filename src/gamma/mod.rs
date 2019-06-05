extern crate bit_vec;

use bit_vec::BitVec;

pub fn encode(v: &Vec<u32>, bv: &mut BitVec) -> () {

    fn u_to_bv(x: u32, logn: u32, bv: &mut BitVec) -> () {
        let mut z = x;
        z = z.rotate_right(logn);
        for _ in 0..logn {
            z = z.rotate_left(1);
            bv.push(z % 2 == 1);
        }
    }

   for e in v {
        let r = (*e).leading_zeros();
        for _ in 0..(32 - r as usize -1) {bv.push(false);}
        u_to_bv(*e, 32 - r, bv);
   }
}

pub fn decode(bv: &BitVec, v: &mut Vec<u32>) -> () {

    let mut mode = 0;
    let mut r = 0;
    let mut u: u32 = 1;
    for b in bv {
        if mode == 0 {
            if b {
                if r == 0 {v.push(1);} else {mode = 1;} 
            }
            else {r += 1;}
        }
        else {
            u <<= 1; if b {u += 1;}
            if r > 1 {r -= 1;} else {v.push(u); r = 0; u = 1; mode = 0;}
        }
    }
    
}

