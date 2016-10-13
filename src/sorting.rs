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
