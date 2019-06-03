use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use std::cmp::{max, min};

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
        if let Some(right) = (*ptr).rt {
            (*ptr).rs = (*right).rs + (*right).ls + 1;
            (*ptr).rh = max((*right).lh, (*right).rh) + 1;
        }
        if (*ptr).lh > (*ptr).rh + 1 {
            let mut left = (*ptr).lt.unwrap();
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
            PosTree::rebalance(left);
        }
        else if (*ptr).lh + 1 < (*ptr).rh {
            println!("right tall");
            let mut right = (*ptr).rt.unwrap();
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
            PosTree::rebalance(right);
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
                    let prev = right_most(ptr);
                    let pprev = (*prev).pt.unwrap();
                    (*prev).pt = (*ptr).pt;
                    (*prev).lt = Some(left);
                    (*prev).rt = Some(right);
                    (*pprev).rt = None;
                    (*left).pt = Some(prev);
                    (*right).pt = Some(prev);
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt == Some(ptr) {(*parent).lt = Some(prev);}
                        else {(*parent).rt = Some(prev);}
                    }
                    PosTree::rebalance(pprev);
                }
                else {
                    (*left).pt = (*ptr).pt;
                    (*left).rt = Some(right);
                    (*right).pt = Some(left);
                    if let Some(parent) = (*ptr).pt {
                        if (*parent).lt == Some(ptr) {(*parent).lt = Some(left);}
                        else {(*parent).rt = Some(left);}
                        PosTree::rebalance(parent);
                    }
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

    pub fn position(ptr: *mut Node<T>) -> usize {
        return unsafe {(*ptr).ls + 
            if !(*ptr).lp {if let Some(parent) = (*ptr).pt {PosTree::position(parent) + 1} else {0}} else {0}
        }
    }

    pub fn print(t: PosTree<T>) -> () {
        unsafe fn find_root<T>(ptr: *mut Node<T>) -> *mut Node<T> {
            return if let Some(pt) = (*ptr).pt {find_root(pt)} else {ptr}
        }
        unsafe fn print_node<T: Debug>(ptr: *mut Node<T>) -> () {
            if let Some(lt) = (*ptr).lt {print_node(lt);}
            print!("v:{:?}, lh:{}, rh:{}, ls: {}, rs: {} |\n ", (*ptr).val, (*ptr).lh, (*ptr).rh, (*ptr).ls, (*ptr).rs);
            if let Some(rt) = (*ptr).rt {print_node(rt);}
        }
        print!("[");
        if let Some(head) = t.head {unsafe {print_node(find_root(head));}}
        println!("]");
    }
}

pub fn convert<T: Eq + Copy + Clone + Hash + Debug>(v: &Vec<T>, w: &mut Vec<usize>, z: &mut Vec<T>) -> () {
    let mut t: PosTree<T> = PosTree::new();
    let mut h: HashMap<T, *mut Node<T>> = HashMap::new();
    for i in 0..v.len() {
        if let Some(ptr) = h.get(&v[i]) {
            w.push(PosTree::position(*ptr));
            unsafe {PosTree::delete(*ptr);}
        }
        else {
            w.push(i + 1);
            z.push(v[i]);
        }
        h.insert(v[i], t.insert(v[i]));
    }
    PosTree::print(t);
}

pub fn restore() -> () {
    {}
}
