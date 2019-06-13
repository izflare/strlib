pub fn encode<T: PartialEq + Copy>(v: &Vec<T>, s: &mut Vec<T>, l: &mut Vec<u32>) -> () {
    
    for c in v {
        if let Some(slast) = s.last() {
            if *slast == *c {
                if let Some(llast) = l.last_mut() {
                    *llast += 1;
                    continue;
                }
            }
        }
        s.push(*c); 
        l.push(1);
    }
}

pub fn decode<T: Copy>(s: &Vec<T>, l: &Vec<u32>, v: &mut Vec<T>) -> () {

    for (c, len) in s.iter().zip(l.iter()) {
        for _ in 0..*len {
            v.push(*c);
        }
    }

}

