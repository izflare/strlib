extern crate bit_vec;

use bit_vec::BitVec;
use super::gamma;
use super::fble;
use super::run_length;

pub fn encode(v: &Vec<u32>, blocksize: u32, bv: &mut BitVec) -> () {

    let mut bbs: Vec<u32> = Vec::new();
    let mut delta: Vec<u32> = Vec::new();
    let mut pms: BitVec = BitVec::new();

    for i in 0..v.len() {
        let bitlength = std::cmp::max(32 - v[i].leading_zeros(), 1);
        if i % blocksize as usize == 0 {bbs.push(bitlength)}
        else {
            if let Some(last) = bbs.last_mut() {
                if bitlength > *last {*last = bitlength;}
            }
        }
    }

    let mut last = 0;
    for bs in &bbs {
        delta.push(if *bs > last {*bs - last} else {last - *bs + 1});
        pms.push(*bs > last);
        last = *bs;
    }

    let mut r1_h: Vec<u32> = Vec::new();
    let mut r1_l: Vec<u32> = Vec::new();
    run_length::encode(&delta, &mut r1_h, &mut r1_l);
    let mut r1_h_bv = BitVec::new();
    gamma::encode(&r1_h, &mut r1_h_bv);

    let mut r2_h: Vec<u32> = Vec::new();
    let mut r2_l: Vec<u32> = Vec::new();
    run_length::encode(&r1_l, &mut r2_h, &mut r2_l);
    let mut r2_h_bv = BitVec::new();
    let mut r2_l_bv = BitVec::new();
    gamma::encode(&r2_h, &mut r2_h_bv);
    gamma::encode(&r2_l, &mut r2_l_bv);

    // write to bv
    fble::to_bv(blocksize, 32, bv);
    fble::to_bv(pms.len() as u32, 32, bv);
    fble::to_bv(r1_h_bv.len() as u32, 32, bv);
    fble::to_bv(r2_h_bv.len() as u32, 32, bv);
    fble::to_bv(r2_l_bv.len() as u32, 32, bv);

    for b in &pms {bv.push(b);}
    for b in &r1_h_bv {bv.push(b);}
    for b in &r2_h_bv {bv.push(b);}
    for b in &r2_l_bv {bv.push(b);}
    let mut r = 0;
    for i in 0..v.len() {
        fble::to_bv(v[i], bbs[r], bv);
        if i as u32 % blocksize == blocksize - 1 {r += 1;}
    }

}


pub fn decode(bv: &BitVec, v: &mut Vec<u32>) -> () {

    let mut blocksize: u32 = 0;
    let mut pms_len: u32 = 0;
    let mut r1_h_bv_len: u32 = 0;
    let mut r2_h_bv_len: u32 = 0;
    let mut r2_l_bv_len: u32 = 0;

    let mut buf: BitVec = BitVec::new();
    let mut bbs: Vec<u32> = Vec::new();
    let mut delta: Vec<u32> = Vec::new();
    let mut pms: BitVec = BitVec::new();

    let mut r1_h: Vec<u32> = Vec::new();
    let mut r1_l: Vec<u32> = Vec::new();
    let mut r1_h_bv = BitVec::new();

    let mut r2_h: Vec<u32> = Vec::new();
    let mut r2_l: Vec<u32> = Vec::new();
    let mut r2_h_bv = BitVec::new();
    let mut r2_l_bv = BitVec::new();

    // parsing
    for i in 0..bv.len() {
        if i < 32 {blocksize <<= 1; if bv[i] {blocksize += 1;}}
        else if i < 32 * 2 {pms_len <<= 1; if bv[i] {pms_len += 1;}}
        else if i < 32 * 3 {r1_h_bv_len <<= 1; if bv[i] {r1_h_bv_len += 1;}}
        else if i < 32 * 4 {r2_h_bv_len <<= 1; if bv[i] {r2_h_bv_len += 1;}}
        else if i < 32 * 5 {r2_l_bv_len <<= 1; if bv[i] {r2_l_bv_len += 1;}}
        else if i < 32 * 5 + pms_len as usize {pms.push(bv[i]);}
        else if i < 32 * 5 + (pms_len + r1_h_bv_len) as usize {r1_h_bv.push(bv[i]);}
        else if i < 32 * 5 + (pms_len + r1_h_bv_len + r2_h_bv_len) as usize {r2_h_bv.push(bv[i]);}
        else if i < 32 * 5 + (pms_len + r1_h_bv_len + r2_h_bv_len + r2_l_bv_len) as usize {r2_l_bv.push(bv[i]);}
        else {buf.push(bv[i]);}
    }

    gamma::decode(&r2_l_bv, &mut r2_l);
    gamma::decode(&r2_h_bv, &mut r2_h);
    run_length::decode(&r2_h, &r2_l, &mut r1_l);

    gamma::decode(&r1_h_bv, &mut r1_h);
    run_length::decode(&r1_h, &r1_l, &mut delta);


    let mut last = 0;
    for (d, b) in delta.iter().zip(pms.iter()) {
        bbs.push(if b {last + *d} else {last - *d + 1});
        last = *bbs.last().unwrap();
    }

    let mut r = 0;
    let mut cnt = 0;
    let mut u: u32 = 0;
    let mut sum = 0;
    for i in 0..buf.len() {
        u <<= 1; if buf[i] {u += 1;}
        if (i - sum) as u32 % bbs[r] == bbs[r] - 1 {v.push(u); u = 0; cnt += 1;}
        if cnt == blocksize {r += 1; sum = i + 1; cnt = 0;}
    }

}

