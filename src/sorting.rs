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

fn heapify<T>(slice: &mut [T], idx: usize, max: usize)
    where T: Ord
{
    let mut largest = idx;
    let left = idx*2 + 1;
    let right = idx*2 + 2;

    if left < max && slice[left] > slice[largest] {
        largest = left;
    }

    if right < max && slice[right] > slice[largest] {
        largest = right;
    }

    if largest != idx {
        slice.swap(largest, idx);
        heapify(slice, largest, max);
    }
}

pub fn heap_sort<T>(slice: &mut [T])
    where T: Ord
{
    let len = slice.len();
    if len <= 1 {
        return;
    }
    for i in (0..len/2).rev() {
        heapify(slice, i, len);
    }
    for i in (1..len).rev() {
        slice.swap(0, i);
        heapify(slice, 0, i);
    }
}

/// Return the index of the median value of the three first elements of a slice.
fn median_of_three_pivot<T>(slice: &mut [T]) -> (usize)
    where T: Ord
{
    if slice[0] > slice[1] {
        if slice[1] > slice[2] {
            // slice[0] > slice[1] > slice[2]
            1
        } else if slice[0] > slice[2] {
            // slice[0] > slice[2] >= slice[1]
            2
        } else {
            // slice[2] >= slice[0] > slice[1]
            0
        }
    } else {
        if slice[0] > slice[2] {
            // slice[1] >= slice[0] > slice[2]
            0
        } else if slice[2] > slice[1] {
            // slice[2] > slice[1] >= slice[0]
            1
        } else {
            // slice[0] <= slice[2] <= slice[1]
            2
        }
    }
}

fn partition<T>(slice: &mut [T], pivot_idx: usize) -> (usize)
    where T: Ord
{
    if slice.len() == 0 {
        panic!("Cannot partition empty slice");
    } else if slice.len() == 1 {
        return 0;
    }
    let mut left: usize = 0;
    let mut right: usize = slice.len() - 1 ;

    // put the pivot at the end so that we know where it is
    slice.swap(right, pivot_idx);
    let pivot_idx = right;
    right -= 1;

    // initialization:
    // pick the first element from the right that is smaller than the pivot
    while right > 0 {
        if slice[right] < slice[pivot_idx] {
            break
        }
        right -= 1;
    }

    // actual partitioning:
    //  - find the first element from the left that it smaller than the pivot
    //  - find the firsh element from the right that is bigger than the pivot
    //  - swap the right and left elements
    while left < right {
        if slice[left] > slice[pivot_idx] {
            slice.swap(left, right);
            right -= 1;
        }
        left += 1;
        while left < right {
            if slice[right] < slice[pivot_idx] {
                break
            }
            right -= 1;
        }
    }

    if slice[left] > slice[pivot_idx] {
        slice.swap(left, pivot_idx);
    }
    left
}

pub fn qsort<T>(values: &mut [T])
    where T: Ord
{
    let slice_len = values.len();
    if slice_len < 5 {
        insertion_sort(values);
    } else {
        let mut pivot = median_of_three_pivot(values);
        pivot = partition(values, pivot);
        qsort(&mut values[0..pivot+1]);
        qsort(&mut values[pivot+1..slice_len]);
    }
}

#[test]
fn test_heapify() {
    let mut values = [8, 11, 9, 2, 10, 16];
    heapify(&mut values, 2, 6);
    assert_eq!(values, [8, 11, 16, 2, 10, 9]);
    heapify(&mut values, 1, 6);
    assert_eq!(values, [8, 11, 16, 2, 10, 9]);
    heapify(&mut values, 0, 6);
    assert_eq!(values, [16, 11, 9, 2, 10, 8]);
}

#[test]
fn test_partition() {
    let mut values = [8, 11, 9, 2, 10, 16];
    let mut pivot = 2;
    pivot = partition(&mut values, pivot);
    assert_eq!(pivot, 2);
    for i in 0..3 {
        assert!(values[i] <= 9, "");
    }
    for i in 3..values.len() {
        assert!(values[i] >= 9);
    }
}

#[test]
fn test_partition_duplicate() {
    let mut values = [8, 8, 11, 11, 9, 9, 2, 2, 10, 10, 16, 16];
    let mut pivot = 5;
    pivot = partition(&mut values, pivot);
    assert_eq!(pivot, 4);
    for i in 0..5 {
        assert!(values[i] <= 9, "");
    }
    for i in 5..values.len() {
        assert!(values[i] >= 9);
    }
}

#[test]
fn test_partition_two_elements() {
    let mut values = [2, 1];
    assert!(partition(&mut values, 1) == 0);
    assert_eq!(values, [1, 2]);

    let mut values = [1, 2];
    assert!(partition(&mut values, 1) == 0);
    assert_eq!(values, [1, 2]);
}
