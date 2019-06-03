extern crate bit_vec;

use bit_vec::BitVec;
use std::time::Instant;
use std::cmp;

fn u_to_bv(x: u32, logn: u32, bv: &mut BitVec) -> () {
    let mut z = x;
    z = z.rotate_right(logn);
    for _ in 0..logn {
        z = z.rotate_left(1);
        bv.push(z % 2 == 1);
    }
}

//{{{
fn gamma_enc(x: u32, bv: &mut BitVec) -> () {
    let r = x.leading_zeros();
    for _ in 0..(32 - r as usize -1) {bv.push(false);}
    u_to_bv(x, 32 - r, bv);
}

fn delta_enc(v: &Vec<u32>, bv: &mut BitVec) -> () {
   for e in v {
        let r = 32 - (*e).leading_zeros();
        let mut d = BitVec::new();
        gamma_enc(r, &mut d);
        u_to_bv(*e, r - 1, &mut d);
        for b in d {bv.push(b);}
   }
}

fn delta_dec(bv: &BitVec, v: &mut Vec<u32>) -> () {
    let mut mode = 0;
    let mut r = 0;
    let mut u: u32 = 1;
    let mut x: u32 = 1;
    for b in bv {
        if mode == 0 {
            if b {
                if r == 0 {v.push(1);} else {mode = 1;} 
            }
            else {r += 1;}
        }
        else if mode == 1 {
            u <<= 1; if b {u += 1;}
            if r > 1 {r -= 1;} else {mode = 2;}
        }
        else {
            x <<= 1; if b {x += 1;}
            if u > 2 {u -= 1;} else {v.push(x); r = 0; u = 1; x = 1; mode = 0;}
        }
    }
}
//}}}

pub fn encode(z: &Vec<u8>, g: &Vec<(u32, u32)>, s: &Vec<u32>, bv: &mut BitVec) -> () {
    //{{{
    // let start = Instant::now();


    println!("----------------------------------------");
    for e in z {u_to_bv(*e as u32, 8, bv);}
    u_to_bv(0, 8, bv);
    let mut v: Vec<u32> = Vec::new();
    let mut r = 0;
    let mut prev = 0;
    let mut inc = true;
    let mut rmax = 0;
    let mut runs: Vec<u32> = Vec::new();
    let mut def: Vec<u32> = Vec::new();
    let mut which: BitVec = BitVec::new();
    for i in 0..g.len() {
        let e = g[i];
        let a = cmp::min(e.0, e.1);
        let b = cmp::max(e.0, e.1);
        let c = b - a + 1;
        r += 1;
        if inc {
            if a >= prev {
                v.push(a - prev + 1);
                prev = a;
            }
            else if b >= prev {
                v.push(b - prev + 1);
                prev = b;
            }
            else {
                v.push(prev - b + 1);
                prev = b;
                runs.push(r);
                if r > rmax {rmax = r;}
                r = 0;
                inc = false;
            }
            which.push(e.0 < e.1);
            def.push(c);
        }
        else {
            if b <= prev {
                v.push(prev - b + 1);
                prev = b;
            }
            else if a <= prev {
                v.push(prev - a + 1);
                prev = a;
            }
            else {
                v.push(a - prev + 1);
                prev = a;
                runs.push(r);
                if r > rmax {rmax = r;}
                r = 0;
                inc = true;
            }
            which.push(e.0 < e.1);
            def.push(c);
        }
    }
    if r > rmax {rmax = r;}
    runs.push(r);
    // println!("{:?}", v);
    // println!("{:?}", runs);
    // println!("{:?}", def);
    // println!("{:?}", which);

    let d = (std::usize::MAX.count_ones() - g.len().leading_zeros()) as usize;
    println!("d: {}", d);
    let mut dummy = BitVec::new();
    delta_enc(&v, &mut dummy);
    // delta_enc(&runs, &mut dummy);
    println!("{:?}", rmax);
    println!("{:?}", runs.len() * 7);
    delta_enc(&def, &mut dummy);
    delta_enc(s, &mut dummy);
    println!("{:?} [bytes]", (bv.len() + dummy.len() + which.len()) / 8);

    println!("----------------------------------------");
    // let end = start.elapsed();
    // println!("[Result: bit encoding]");
    // println!("B length          : {:?} [bits]", b.len());
    // println!("L length          : {:?} [words]", l.len());
    // println!("log (n + sigma)   : {:?}", logn);
    // println!("{}.{:03} sec elapsed", end.as_secs(), end.subsec_nanos()/1_000_000);

    //}}}
}

pub fn decode(bv: &BitVec, w: &mut Vec<u8>) -> () {
    //{{{
    
    let mut mode = 1;
    let mut t = 0;
    let mut i = 0;
    let mut b: BitVec = BitVec::new();
    let mut u: u32 = 0;
    let mut z: Vec<u8> = Vec::new();
    let mut logn: u32 = 0;
    let mut l: Vec<u32> = Vec::new();
    // let mut d: BitVec = BitVec::new();
    for bit in bv {
        if mode == 1 {
            b.push(bit);
            if bit {t -= 1;} else {t += 1;}
            if t == 0 {mode = 2;}
        }
        else if mode == 2 {
            u <<= 1; if bit {u += 1;} i += 1;
            if i >= 8 {
                if u == 0 {mode = 3; i = 0;}
                else {z.push(u as u8); u = 0; i = 0;}
            }
        }
        else if mode == 3 {
            u <<= 1; if bit {u += 1;} i += 1;
            if i >= 8 {logn = u as u32; u = 0; mode = 4; i = 0;}
        }
        else {
            u <<= 1; if bit {u += 1;} i += 1;
            if i >= logn {l.push(u as u32); u = 0; i = 0;}
        }
        // else {
        //     d.push(bit);
        // }
    }
    // delta_dec(&d, &mut l);
    
    let mut dec_g: Vec<(u32, u32)> = Vec::new();
    fn dec_drv(x: u32, dec_g: &Vec<(u32, u32)>, z: &Vec<u8>, w: &mut Vec<u8>) -> () {
        if x as usize <= z.len() {w.push(z[x as usize -1]);}
        else {
            let bg = dec_g[x as usize - z.len() -1];
            dec_drv(bg.0, dec_g, z, w);
            dec_drv(bg.1, dec_g, z, w);
        }
    }

    let mut dec_i = 0;
    let mut dec_x = z.len() as u32 + 1;
    let mut stack: Vec<u32> = Vec::new();
    for dec_b in &b {
        if dec_b {
            if let Some(rt) = stack.pop() {
                if let Some(lt) = stack.pop() {
                    dec_g.push((lt, rt));
                }
            }
            stack.push(dec_x);
            dec_x += 1;
        }
        else {
            stack.push(l[dec_i]);
            dec_drv(l[dec_i], &dec_g, &z, w);
            dec_i += 1;
        }
    }
    //}}}
}

