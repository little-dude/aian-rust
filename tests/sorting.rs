extern crate aian;
#[macro_use]
extern crate quickcheck;

use aian::sorting::insertion_sort;
use aian::sorting::heap_sort;

#[test]
fn insertion_sort_empty() {
    let mut values: [i32; 0] = [];
    insertion_sort(&mut values);
    assert_eq!(values, [])
}

#[test]
fn insertion_sort_one() {
    let mut values = [1];
    insertion_sort(&mut values);
    assert_eq!(values, [1]);
}

#[test]
fn insertion_multi() {
    let mut values = [9, 8, 7, 11, 10];
    insertion_sort(&mut values);
    let values_expected: Vec<_> = (7..12).collect();
    assert_eq!(values_expected, values);
}

quickcheck! {
    fn insertion_everything(xs: Vec<i32>) -> bool {
        // Macro doesn't allow `mut` in the `fn` declaration :-(
        let mut xs = xs;

        let mut expected_sorted = xs.clone();
        expected_sorted.sort();

        insertion_sort(&mut xs);

        expected_sorted == xs
    }
}

#[test]
fn heap_sort_empty() {
    let mut values: [i32; 0] = [];
    heap_sort(&mut values);
    assert_eq!(values, [])
}

#[test]
fn heap_sort_one() {
    let mut values = [1];
    heap_sort(&mut values);
    assert_eq!(values, [1]);
}

#[test]
fn heap_multi() {
    let mut values = [9, 8, 7, 11, 10];
    heap_sort(&mut values);
    let values_expected: Vec<_> = (7..12).collect();
    assert_eq!(values_expected, values);
}

quickcheck! {
    fn heap_everything(xs: Vec<i32>) -> bool {
        let mut xs = xs;
        let mut expected_sorted = xs.clone();
        expected_sorted.sort();
        heap_sort(&mut xs);
        expected_sorted == xs
    }
}
