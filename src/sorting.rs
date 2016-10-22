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
