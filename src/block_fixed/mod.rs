extern crate bit_vec;

use bit_vec::BitVec;
use super::gamma;
use super::fixed;
use super::runlength;

pub fn encode(v: &Vec<u32>, width: u32, bv: &mut BitVec) -> () {

    let mut bbs: Vec<u32> = Vec::new();
    let mut delta: Vec<u32> = Vec::new();
    let mut pms: BitVec = BitVec::new();

    for i in 0..v.len() {
        let bitlength = 32 - v[i].leading_zeros();
        if i % width as usize == 0 {bbs.push(bitlength)}
        else {
            if let Some(last) = bbs.last_mut() {
                if bitlength > *last {*last += 1;}
            }
        }
    }

    let mut last = 0;
    for bs in &bbs {
        delta.push(if *bs > last {*bs - last} else {last - *bs});
        pms.push(*bs > last);
        last = *bs;
    }

    let mut r1_s: Vec<u32> = Vec::new();
    let mut r1_l: Vec<u32> = Vec::new();
    runlength::encode(&delta, &mut r1_s, &mut r1_l);
    let mut r1_s_bv = BitVec::new();
    gamma::encode(&r1_s, &mut r1_s_bv);

    let mut r2_s: Vec<u32> = Vec::new();
    let mut r2_l: Vec<u32> = Vec::new();
    runlength::encode(&r1_l, &mut r2_s, &mut r2_l);
    let mut r2_s_bv = BitVec::new();
    let mut r2_l_bv = BitVec::new();
    gamma::encode(&r2_s, &mut r2_s_bv);
    gamma::encode(&r2_l, &mut r2_l_bv);

    // write to bv
    fixed::to_bv(width, 32, bv);
    fixed::to_bv(pms.len() as u32, 32, bv);
    fixed::to_bv(r1_s_bv.len() as u32, 32, bv);
    fixed::to_bv(r2_s_bv.len() as u32, 32, bv);
    fixed::to_bv(r2_l_bv.len() as u32, 32, bv);

    for b in &pms {bv.push(b);}
    for b in &r1_s_bv {bv.push(b);}
    for b in &r2_s_bv {bv.push(b);}
    for b in &r2_l_bv {bv.push(b);}
    let mut r = 0;
    for i in 0..v.len() {
        fixed::to_bv(v[i], bbs[r], bv);
        if i as u32 % width == width - 1 {r += 1;}
    }

}


pub fn decode(bv: &BitVec, v: &mut Vec<u32>) -> () {

    let mut width: u32 = 0;
    let mut pms_len: u32 = 0;
    let mut r1_s_bv_len: u32 = 0;
    let mut r2_s_bv_len: u32 = 0;
    let mut r2_l_bv_len: u32 = 0;

    let mut buf: BitVec = BitVec::new();
    let mut bbs: Vec<u32> = Vec::new();
    let mut delta: Vec<u32> = Vec::new();
    let mut pms: BitVec = BitVec::new();

    let mut r1_s: Vec<u32> = Vec::new();
    let mut r1_l: Vec<u32> = Vec::new();
    let mut r1_s_bv = BitVec::new();

    let mut r2_s: Vec<u32> = Vec::new();
    let mut r2_l: Vec<u32> = Vec::new();
    let mut r2_s_bv = BitVec::new();
    let mut r2_l_bv = BitVec::new();

    // parsing
    for i in 0..bv.len() {
        if i < 32 {width <<= 1; if bv[i] {width += 1;}}
        else if i < 32 * 2 {pms_len <<= 1; if bv[i] {pms_len += 1;}}
        else if i < 32 * 3 {r1_s_bv_len <<= 1; if bv[i] {r1_s_bv_len += 1;}}
        else if i < 32 * 4 {r2_s_bv_len <<= 1; if bv[i] {r2_s_bv_len += 1;}}
        else if i < 32 * 5 {r2_l_bv_len <<= 1; if bv[i] {r2_l_bv_len += 1;}}
        else if i < 32 * 5 + pms_len as usize {pms.push(bv[i]);}
        else if i < 32 * 5 + (pms_len + r1_s_bv_len) as usize {r1_s_bv.push(bv[i]);}
        else if i < 32 * 5 + (pms_len + r1_s_bv_len + r2_s_bv_len) as usize {r2_s_bv.push(bv[i]);}
        else if i < 32 * 5 + (pms_len + r1_s_bv_len + r2_s_bv_len + r2_l_bv_len) as usize {r2_l_bv.push(bv[i]);}
        else {buf.push(bv[i]);}
    }

    gamma::decode(&r2_l_bv, &mut r2_l);
    gamma::decode(&r2_s_bv, &mut r2_s);
    runlength::decode(&r2_s, &r2_l, &mut r1_l);

    gamma::decode(&r1_s_bv, &mut r1_s);
    runlength::decode(&r1_s, &r1_l, &mut delta);

    let mut last = 0;
    for (d, b) in delta.iter().zip(pms.iter()) {
        bbs.push(if b {last + *d} else {last - *d});
        last = *bbs.last().unwrap();
    }

    let mut r = 0;
    let mut cnt = 0;
    let mut u: u32 = 0;
    let mut sum = 0;
    for i in 0..buf.len() {
        u <<= 1; if buf[i] {u += 1;}
        if (i - sum) as u32 % bbs[r] == bbs[r] - 1 {v.push(u); u = 0; cnt += 1;}
        if cnt == width {r += 1; sum = i + 1; cnt = 0;}
    }

}

