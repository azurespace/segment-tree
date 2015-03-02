use std::marker::{PhantomData,  Sized}; 
use std::num::Int;

pub trait SegmentTreeOps<T: Sized>: Sized{
    fn get(accumulator: T, node_value: T) -> T;
    fn calc(left_value: T, right_value: T) -> T;
    fn default_acc() -> T;
}

pub struct add_ops<T:Int>{
    f: PhantomData<T>,
}

impl<T:Int> SegmentTreeOps<T> for add_ops<T>{
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


pub struct max_ops<T:Int>{
    f: PhantomData<T>,
}

impl<T:Int> SegmentTreeOps<T> for max_ops<T>{
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

pub struct min_ops<T:Int>{
    f: PhantomData<T>,
}

impl<T:Int> SegmentTreeOps<T> for min_ops<T>{
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
