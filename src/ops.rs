extern crate num;
use std::marker::{PhantomData,  Sized}; 
use num::integer::Integer;
use num::traits::Bounded;

pub trait SegmentTreeOps<T: Sized>: Sized{
    fn get(accumulator: T, node_value: T) -> T;
    fn calc(left_value: T, right_value: T) -> T;
    fn default_acc() -> T;
}


pub struct AddOps<T:Integer+Bounded>{
    f: PhantomData<T>,
}

impl<T:Integer+Bounded> SegmentTreeOps<T> for AddOps<T>{
    fn get(accumulator: T, node_value: T) -> T{
        accumulator + node_value
    }
    fn calc(left_value: T, right_value: T) -> T{
        left_value + right_value
    }
    fn default_acc() -> T {
        T::zero()
    }
}


pub struct MaxOps<T:Integer+Bounded>{
    f: PhantomData<T>,
}

impl<T:Integer+Bounded> SegmentTreeOps<T> for MaxOps<T>{
    fn get(accumulator: T, node_value: T) -> T{
        if accumulator < node_value{
            node_value
        } else {
            accumulator
        }
    }
    fn calc(left_value: T, right_value: T) -> T{
        if left_value < right_value{
            right_value
        } else {
            left_value
        }
    }
    fn default_acc() -> T {
        T::min_value()
    }
}

pub struct MinOps<T:Integer>{
    f: PhantomData<T>,
}

impl<T:Integer+Bounded> SegmentTreeOps<T> for MinOps<T>{
    fn get(accumulator: T, node_value: T) -> T{
        if accumulator > node_value{
            node_value
        } else {
            accumulator
        }
    }
    fn calc(left_value: T, right_value: T) -> T{
        if left_value > right_value{
            right_value
        } else {
            left_value
        }
    }
    fn default_acc() -> T {
        T::max_value()
    }
}
