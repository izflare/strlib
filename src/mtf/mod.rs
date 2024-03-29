use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::cmp::max;

pub fn convert<T: Eq + Copy + Clone + Hash + Debug>(v: &Vec<T>, w: &mut Vec<usize>, z: &mut Vec<T>) -> () {

    #[derive(Debug)]
    pub struct Node<T> {
        val: T, 
        lh: usize, rh: usize, ls: usize, rs: usize,
        lp: bool,
        ht: Option<*mut PosTree<T>>, pt: Option<*mut Node<T>>, 
        lt: Option<*mut Node<T>>, rt: Option<*mut Node<T>>, 
    }

    #[derive(Clone, Copy)]
    pub struct PosTree<T> {
        head: Option<*mut Node<T>>,
    }

    impl<T: Debug> PosTree<T> {
        pub fn new() -> Self {
            Self {head: None,}
        }

        pub fn insert(&mut self, x: T) -> *mut Node<T> {
            let ptr = Box::into_raw(Box::new(
                    Node {val: x, ht: Some(self), pt: self.head, lt: None, rt: None, 
                        lh: 0, rh: 0, ls: 0, rs: 0, lp: true}));
            if let Some(head) = self.head {
                unsafe {
                    (*head).lt = Some(ptr);
                    (*head).ht = None
                }
            }
            self.head = Some(ptr);
            unsafe {PosTree::rebalance(ptr);}
            return ptr;
        }

        pub unsafe fn rebalance(ptr: *mut Node<T>) -> () {
            //{{{
            if let Some(left) = (*ptr).lt {
                (*ptr).ls = (*left).ls + (*left).rs + 1;
                (*ptr).lh = max((*left).lh, (*left).rh) + 1;
            }
            else {
                (*ptr).ls = 0;
                (*ptr).lh = 0;
            }
            if let Some(right) = (*ptr).rt {
                (*ptr).rs = (*right).rs + (*right).ls + 1;
                (*ptr).rh = max((*right).lh, (*right).rh) + 1;
            }
            else {
                (*ptr).rs = 0;
                (*ptr).rh = 0;
            }
            if (*ptr).lh > (*ptr).rh + 1 {
                let mut left = (*ptr).lt.unwrap();
                if (*left).lh < (*left).rh {
                    let mut leftr = (*left).rt.unwrap();
                    (*leftr).pt = (*ptr).pt;
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt.unwrap() == ptr {(*parent).lt = Some(leftr);}
                        else {(*parent).rt = Some(leftr);}
                    }
                    (*ptr).lt = (*leftr).rt;
                    (*ptr).ls = 0;
                    (*ptr).lh = 0;
                    if let Some(leftrr) = (*leftr).rt {
                        (*leftrr).pt = Some(ptr);
                        (*ptr).ls = (*leftrr).ls + (*leftrr).rs + 1;
                        (*ptr).lh = max((*leftrr).ls, (*leftrr).rs) + 1;
                    }
                    (*left).rt = (*leftr).lt;
                    (*left).rs = 0;
                    (*left).rh = 0;
                    if let Some(leftrl) = (*leftr).lt {
                        (*leftrl).pt = Some(left);
                        (*left).rs = (*leftrl).ls + (*leftrl).rs + 1;
                        (*left).rh = max((*leftrl).ls, (*leftrl).rs) + 1;
                    }
                    (*ptr).pt = Some(leftr);
                    (*left).pt = Some(leftr);
                    (*leftr).rt = Some(ptr);
                    (*leftr).lt = Some(left);
                    if (*ptr).lp {(*leftr).lp = true;}
                    (*ptr).lp = false;
                    PosTree::rebalance(leftr);
                }
                else {
                    (*left).pt = (*ptr).pt;
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt.unwrap() == ptr {(*parent).lt = Some(left);}
                        else {(*parent).rt = Some(left);}
                    }
                    (*ptr).lt = (*left).rt;
                    (*ptr).ls = 0;
                    (*ptr).lh = 0;
                    if let Some(leftr) = (*left).rt {
                        (*leftr).pt = Some(ptr);
                        (*ptr).ls = (*leftr).ls + (*leftr).rs + 1;
                        (*ptr).lh = max((*leftr).ls, (*leftr).rs) + 1;
                    }
                    (*ptr).pt = Some(left);
                    (*left).rt = Some(ptr);
                    (*ptr).lp = false;
                    PosTree::rebalance(ptr);
                }
            }
            else if (*ptr).lh + 1 < (*ptr).rh {
                let mut right = (*ptr).rt.unwrap();
                if (*right).lh > (*right).rh {
                    let mut rightl = (*right).lt.unwrap();
                    (*rightl).pt = (*ptr).pt;
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt.unwrap() == ptr {(*parent).lt = Some(rightl);}
                        else {(*parent).rt = Some(rightl);}
                    }
                    (*ptr).rt = (*rightl).lt;
                    (*ptr).rs = 0;
                    (*ptr).rh = 0;
                    if let Some(rightll) = (*rightl).lt {
                        (*rightll).pt = Some(ptr);
                        (*ptr).rs = (*rightll).ls + (*rightll).rs + 1;
                        (*ptr).rh = max((*rightll).ls, (*rightll).rs) + 1;
                    }
                    (*right).lt = (*rightl).rt;
                    (*right).ls = 0;
                    (*right).lh = 0;
                    if let Some(rightlr) = (*rightl).rt {
                        (*rightlr).pt = Some(right);
                        (*right).ls = (*rightlr).ls + (*rightlr).rs + 1;
                        (*right).lh = max((*rightlr).ls, (*rightlr).rs) + 1;
                    }
                    (*ptr).pt = Some(rightl);
                    (*right).pt = Some(rightl);
                    (*rightl).lt = Some(ptr);
                    (*rightl).rt = Some(right);
                    if (*ptr).lp {(*rightl).lp = true;}
                    PosTree::rebalance(rightl);
                }
                else {
                    (*right).pt = (*ptr).pt;
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).rt.unwrap() == ptr {(*parent).rt = Some(right);}
                        else {(*parent).lt = Some(right);}
                    }
                    (*ptr).rt = (*right).lt;
                    (*ptr).rs = 0;
                    (*ptr).rh = 0;
                    if let Some(rightl) = (*right).lt {
                        (*rightl).pt = Some(ptr);
                        (*ptr).rs = (*rightl).ls + (*rightl).rs + 1;
                        (*ptr).rh = max((*rightl).ls, (*rightl).rs) + 1;
                    }
                    (*ptr).pt = Some(right);
                    (*right).lt = Some(ptr);
                    if (*ptr).lp {(*right).lp = true;}
                    PosTree::rebalance(ptr);
                }
            }
            else if let Some(parent) = (*ptr).pt {
                PosTree::rebalance(parent);
            }
            //}}}
        }

        pub unsafe fn delete(ptr: *mut Node<T>) -> () {
            //{{{
            if let Some(left) = (*ptr).lt {
                if let Some(right) = (*ptr).rt {
                    if let Some(_) = (*left).rt {
                        unsafe fn right_most<T>(ptr: *mut Node<T>) -> *mut Node<T> {
                            if let Some(rt) = (*ptr).rt {return right_most(rt);}
                            else {return ptr;}
                        }
                        let prev = right_most(left);
                        let pprev = (*prev).pt.unwrap();
                        (*pprev).rt = (*prev).lt;
                        (*prev).pt = (*ptr).pt;
                        (*prev).lt = Some(left);
                        (*prev).rt = Some(right);
                        (*left).pt = Some(prev);
                        (*right).pt = Some(prev);
                        if let Some(parent) = (*ptr).pt {
                            if (*parent).lt == Some(ptr) {(*parent).lt = Some(prev);}
                            else {(*parent).rt = Some(prev);}
                        }
                        if (*ptr).lp {(*prev).lp = true;}
                        PosTree::rebalance(pprev);
                    }
                    else {
                        (*left).pt = (*ptr).pt;
                        (*left).rt = Some(right);
                        (*right).pt = Some(left);
                        if let Some(parent) = (*ptr).pt {
                            if (*parent).lt == Some(ptr) {(*parent).lt = Some(left);}
                            else {(*parent).rt = Some(left);}
                        }
                        PosTree::rebalance(left);
                    }
                }
                else {
                    (*left).pt = (*ptr).pt;
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt == Some(ptr) {(*parent).lt = Some(left);}
                        else {(*parent).rt = Some(left);}
                        PosTree::rebalance(parent);
                    }
                }
            }
            else {
                if let Some(right) = (*ptr).rt {
                    (*right).pt = (*ptr).pt;
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt == Some(ptr) {(*parent).lt = Some(right);}
                        else {(*parent).rt = Some(right);}
                        PosTree::rebalance(parent);
                    }
                    if (*ptr).lp {(*right).lp = true;}
                }
                else {
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt == Some(ptr) {(*parent).lt = None;}
                        else {(*parent).rt = None;}
                        PosTree::rebalance(parent);
                    }
                    if let Some(root) = (*ptr).ht {(*root).head = (*ptr).pt;}
                }
            }
        //}}}
        }

        pub unsafe fn position(ptr: *mut Node<T>) -> usize {
            if (*ptr).lp {return (*ptr).ls + 1;}
            else {
                let parent = (*ptr).pt.unwrap();
                return PosTree::position(parent) + if (*parent).lt == Some(ptr) {(*ptr).ls - 1} else {(*ptr).ls +  1};
            }
        }

        // pub fn print(t: PosTree<T>) -> () {
        //     //{{{
        //     unsafe fn find_root<T>(ptr: *mut Node<T>) -> *mut Node<T> {
        //         return if let Some(pt) = (*ptr).pt {find_root(pt)} else {ptr}
        //     }
        //     unsafe fn print_node<T: Debug>(ptr: *mut Node<T>) -> () {
        //         if let Some(lt) = (*ptr).lt {print_node(lt);}
        //         print!("v:{:?}, lh:{}, ls:{}, rh: {}, rs: {}, lp: {:?} |\n ", 
        //                (*ptr).val, (*ptr).lh, (*ptr).ls, (*ptr).rh, (*ptr).rs, (*ptr).lp);
        //         if let Some(rt) = (*ptr).rt {print_node(rt);}
        //     }
        //     print!("[");
        //     if let Some(head) = t.head {unsafe {print_node(find_root(head));}}
        //     println!("]");
        //     //}}}
        // }
    }

    let mut t: PosTree<T> = PosTree::new();
    let mut h: HashMap<T, *mut Node<T>> = HashMap::new();
    for i in 0..v.len() {
        if let Some(ptr) = h.get(&v[i]) {
            w.push(unsafe {PosTree::position(*ptr)});
            unsafe {PosTree::delete(*ptr);}
        }
        else {
            w.push(z.len() + 1);
            z.push(v[i]);
        }
        h.insert(v[i], t.insert(v[i]));
    }
}


pub fn restore<T: Copy + Debug>(v: &Vec<usize>, z: &Vec<T>, w: &mut Vec<T>) -> () {
    let mut j = 0;
    let mut s: Vec<T> = Vec::new();
    for i in 0..v.len() {
        if v[i] > j {w.push(z[j]); s.push(z[j]); j += 1;}
        else {
            let pos = s.len() - v[i];
            w.push(s[pos]);
            s.push(s[pos]);
            s.remove(pos);
        }
    }
}
