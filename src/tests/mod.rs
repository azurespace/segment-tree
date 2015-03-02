extern crate test;
use SegmentTree;
use ops::{add_ops, max_ops, min_ops};

#[test]
fn add_ops(){
    let mut tree: SegmentTree<i32,  add_ops<i32>> = SegmentTree::new(0, 1024);
    tree.set_value(5, 1); // a[5] = 1;
    tree.set_value(6, 1); // a[6] = 1;

    assert_eq!(1, tree.get_range(1,6)); // a[1]+a[2]+a[3]+a[4]+a[5]
    assert_eq!(2, tree.get_range(1,7)); // a[1]+a[2]+a[3]+a[4]+a[5]+a[6]

    tree.set_value(7, 2); // a[7] = 2;
    
    assert_eq!(3, tree.get_range(6,8)); // a[6]+a[7]
    
    tree.set_value(6, 3); // a[6] = 3;
    assert_eq!(4, tree.get_range(5,7)); // a[5]+a[6]
}

#[test]
fn max_ops(){
    let mut tree: SegmentTree<i32,  max_ops<i32>> = SegmentTree::new(0, 1024);
    tree.set_value(5, 1); // a[5] = 1;
    tree.set_value(6, 1); // a[6] = 1;

    assert_eq!(1, tree.get_range(1,6)); // max(a[1..6])
    assert_eq!(1, tree.get_range(1,7)); // max(a[1..7])

    tree.set_value(7, 2); // a[7] = 2;
    
    assert_eq!(2, tree.get_range(6,8)); // max(a[6],a[7])
    
    tree.set_value(6, 3); // a[6] = 3;
    assert_eq!(3, tree.get_range(5,7)); // max(a[5],a[6])

    tree.set_value(1000, 5);
    assert_eq!(5, tree.get_range(0,1024));
}

#[test]
fn min_ops(){
    let mut tree: SegmentTree<i32,  min_ops<i32>> = SegmentTree::new(0, 1024);
    tree.set_value(5, 1); // a[5] = 1;
    tree.set_value(6, 2); // a[6] = 2;

    assert_eq!(0, tree.get_range(1,6)); // min(a[1..6])
    assert_eq!(0, tree.get_range(1,7)); // min(a[1..7])

    tree.set_value(7, 1); // a[7] = 3;
    
    assert_eq!(1, tree.get_range(6,8)); // min(a[6],a[7])
    
    tree.set_value(5, -1); // a[5] = -1;
    tree.set_value(6, 3); // a[6] = 3;
    assert_eq!(-1, tree.get_range(5,7)); // min(a[5],a[6])

    tree.set_value(1000, 5);
    assert_eq!(-1, tree.get_range(0,1024));
}
