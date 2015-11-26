#![crate_type = "lib"]
#![cfg_attr(test, feature(test))]
extern crate num;
use num::integer::Integer;
use num::traits::Bounded;
use std::marker::{PhantomData,  Sized}; 
use node::SegmentTreeNode;
use ops::SegmentTreeOps;

#[cfg(test)]
mod tests;
mod node;
mod ops;
 
pub struct SegmentTree<T: Integer+Copy+Bounded, F: Sized>{
    root: SegmentTreeNode<T>,
    f: PhantomData<F>
}
 
impl<T: Integer+Copy+Bounded, F: SegmentTreeOps<T>> SegmentTree<T, F>
{
    pub fn new(lower_bound: i32, upper_bound: i32) -> SegmentTree<T, F> 
    {
        assert!(lower_bound <= upper_bound);
        SegmentTree::<T,F>{
            root: SegmentTreeNode::<T>::new(lower_bound, upper_bound),
            f: PhantomData::<F>
        }
    }
 
    pub fn get_range(&self, lower_bound: i32, upper_bound: i32) -> T 
    {
        let mut v: Vec<&SegmentTreeNode<T>> = Vec::new();
        let mut acc = F::default_acc();
 
        self.root.find_range(lower_bound, upper_bound, &mut v);
 
        for i in v.iter()
        {
            acc = F::get(acc, i.value.get());
        }
        acc
    }
 
    pub fn get_value(&self, pos: i32) -> T {
        let mut v: Vec<&SegmentTreeNode<T>> = Vec::new();
        self.root.find_pos(pos, &mut v);
 
        v[0].value.get()
    }
 
    pub fn set_value(&mut self, pos: i32, new_value: T) {
        let mut v: Vec<&SegmentTreeNode<T>> = Vec::new();
        self.root.find_pos(pos, &mut v);
 
        v.get_mut(0).unwrap().value.set(new_value);
 
        for node in v.iter_mut().skip(1)
        {
            let &(ref l, ref r) = node.children.as_ref().unwrap();

            node.value.set(F::calc(l.value.get(), r.value.get()));
        }
    }
}
