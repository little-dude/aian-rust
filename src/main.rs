fn main() {
}

pub fn insertion_sort<T>(values: &mut [T])
    where T: Ord
{
    for i in 0..values.len() {
        for j in (0..i).rev() {
            if values[j] >= values[j + 1] {
                values.swap(j, j + 1);
            } else {
                break
            }
        }
    }
}

#[macro_use]
extern crate quickcheck;

#[test]
fn test_insertion_sort_empty() {
    let mut values: [i32; 0] = [];
    insertion_sort(&mut values);
    assert_eq!(values, [])
}

#[test]
fn test_insertion_sort_one() {
    let mut values = [1];
    insertion_sort(&mut values);
    assert_eq!(values, [1]);
}

#[test]
fn test_insertion_multi() {
    let mut values = [9, 8, 7, 11, 10];
    insertion_sort(&mut values);
    let values_expected: Vec<_> = (7..12).collect();
    assert_eq!(values_expected, values);
}

quickcheck! {
    fn test_insertion_everything(xs: Vec<i32>) -> bool {
        // Macro doesn't allow `mut` in the `fn` declaration :-(
        let mut xs = xs;

        let mut expected_sorted = xs.clone();
        expected_sorted.sort();

        insertion_sort(&mut xs);

        expected_sorted == xs
    }
}
