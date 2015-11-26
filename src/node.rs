extern crate num;

use std::cell::Cell;
use num::integer::Integer;

pub struct SegmentTreeNode<T:Copy>
{
    lower_bound: i32,
    upper_bound: i32,
    pub value: Cell<T>,
    pub children: Option<(Box<SegmentTreeNode<T>>, Box<SegmentTreeNode<T>>)>
}
 
impl<T:Integer+Copy> SegmentTreeNode<T>{
    pub fn new(
        lower_bound:i32,
        upper_bound:i32) -> SegmentTreeNode<T>
    {
        let size = upper_bound - lower_bound;
        assert!(size >= 0);
 
        let mut children = None;
 
        if size > 1 
        {
            let mid = (lower_bound + upper_bound) / 2;
            children = Some( 
                (
                    Box::new(SegmentTreeNode::<T>::new(lower_bound, mid)),
                    Box::new(SegmentTreeNode::<T>::new(mid, upper_bound))
                ) 
                );
        }        
 
        SegmentTreeNode::<T>{
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            value: Cell::new(T::zero()),
            children: children
        }
    }
 
    pub fn find_range<'a>(&'a self, lower_bound: i32, upper_bound: i32, v:&mut Vec<&'a SegmentTreeNode<T>>)
    {
        if upper_bound <= self.lower_bound || lower_bound >= self.upper_bound {
            return
        }
 
        if lower_bound <= self.lower_bound && self.upper_bound <= upper_bound {
            v.push(self);
            return
        }
 
        let &(ref l, ref r) = self.children.as_ref().unwrap();
        l.find_range(lower_bound, upper_bound, v);
        r.find_range(lower_bound, upper_bound, v);
    }
 
    pub fn find_pos<'a>(&'a self, pos: i32, v: &mut Vec<&'a SegmentTreeNode<T>>)
    {
        if self.lower_bound > pos || self.upper_bound <= pos {
            return;
        }
 
        if self.lower_bound == pos && self.upper_bound == self.lower_bound + 1 
        {
            v.push(self);
            return;
        }
 
        let mid = (self.lower_bound + self.upper_bound) / 2;    
        let &(ref l, ref r) = self.children.as_ref().unwrap();
        if pos < mid 
        {
            l.find_pos(pos, v)
        }
        else
        {
            r.find_pos(pos, v)
        }
 
        v.push(self);
    }
}
 

