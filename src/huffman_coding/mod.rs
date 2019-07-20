extern crate bit_vec;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use bit_vec::BitVec;

pub fn encode(v: &Vec<u32>, encoded: &mut BitVec) -> () {
    //{{{

    // Huffman tree def
    #[derive(Eq, PartialEq)]
    struct Node {
        val: Option<u32>,
        freq: usize,
        left: Option<*mut Node>,
        right: Option<*mut Node>,
    }
    impl Ord for Node {fn cmp(&self, other: &Node) -> Ordering {other.freq.cmp(&self.freq)}}
    impl PartialOrd for Node {fn partial_cmp(&self, other: &Node) -> Option<Ordering> {Some(self.cmp(other))}}

    // freq counting
    let mut h: HashMap<u32, usize> = HashMap::new();
    for c in v {*h.entry(*c).or_insert(0) += 1;}
    let mut heap = BinaryHeap::new();
    for (k, v) in &h {heap.push(Node{val: Some(*k), freq: *v, left: None, right: None})}

    // tree constructing
    loop {
        if heap.len() < 2 {break;}
        let x = Box::into_raw(Box::new(heap.pop().unwrap()));
        let y = Box::into_raw(Box::new(heap.pop().unwrap()));
        unsafe{heap.push(Node{val:None, freq: (*x).freq + (*y).freq, left: Some(x), right: Some(y)});}
    }

    // encoding
    let root = Box::into_raw(Box::new(heap.pop().unwrap()));
    let code: *mut HashMap<u32, BitVec> = Box::into_raw(Box::new(HashMap::new()));
    let mut treebv: BitVec = BitVec::new();

    fn u32_to_bv(x: u32, bv: &mut BitVec) -> () {
        //{{{
        let mut z = x;
        for _ in 0..32 {
            z = z.rotate_left(1);
            bv.push(z % 2 == 1);
        }
        //}}}
    }

    fn enc(ptr: *mut Node, mut b: BitVec, code: &*mut HashMap<u32, BitVec>, treebv: &mut BitVec) {
        //{{{
        unsafe {
            if let Some(x) = (*ptr).val {
                // println!("({}, {:?})", x, b);
                (**code).insert(x, b);
                (*treebv).push(true);
                u32_to_bv(x, treebv);
            }
            else {
                let mut c = b.clone();
                b.push(false);
                (*treebv).push(false);
                enc((*ptr).left.unwrap(), b, code, treebv);
                c.push(true);
                enc((*ptr).right.unwrap(), c, code, treebv);
            }
        }
        //}}}
    }
    enc(root, BitVec::new(), &code, &mut treebv);
    let mut encbv: BitVec = BitVec::new();
    for c in v {
        let bv = unsafe {(*code).get(c).unwrap()};
        for b in bv {encbv.push(b);}
    }
    // encoded = rbv + treebv.len (64 bits) + treebv + encbv
    for _ in 0..(7 - (treebv.len()+encbv.len()) % 8) {encoded.push(false);}
    encoded.push(true);
    u32_to_bv(h.len() as u32, encoded);
    for b in &treebv {encoded.push(b);}
    for b in &encbv {encoded.push(b);}
    //}}}
}

pub fn decode(bv: &BitVec, v: &mut Vec<u32>) -> () {
    //{{{
    let mut mode = 0;
    let mut t = 0;
    let mut u: u32 = 0;
    let mut code: BitVec = BitVec::new();
    let mut h: HashMap<BitVec, u32> = HashMap::new();
    let mut i = 0;
    for b in bv {
        if mode == 0 {if b {mode = 1;}}
        else if mode == 1 {
            if i < 32 {
                if i != 0 {t <<= 1;}
                if b {t += 1;}
                i += 1;
            }
            if i == 32 {i = 0; mode = 2;}
        }
        else if mode == 2 {
            if b {i = 0; u = 0; mode = 3;}
            else {code.push(false);}
        }
        else if mode == 3 {
            if i < 32 {
                if i != 0 {u <<= 1;}
                if b {u += 1;}
                i += 1;
            }
            if i == 32 {
                h.insert(code.clone(), u);
                // println!("({:?}, {:?})", u, code);
                t -= 1;
                if t > 0 {i = 0; 
                    loop { if let Some(x) = code.pop() {if !x {break;}} }
                    code.push(true); mode = 2;} 
                else {i = 0; mode = 4; code = BitVec::new();}
            }
        }
        else {
            code.push(b);
            match h.get(&code) {
                Some(x) => {
                    v.push(*x);
                    code = BitVec::new();
                },
                None => (),
            }
        }
    }
    //}}}
}

