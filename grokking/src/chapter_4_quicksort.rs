fn quick_sorted<T>(vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    if vec.len() < 2 {
        return vec;
    };

    let pivot_index = vec.len() / 2;
    let pivot_value = vec[pivot_index];

    let (mut lesser_than_pivot, greater_than_pivot): (Vec<T>, Vec<T>) = vec
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i != pivot_index)
        .map(|(_, v)| v)
        .partition(|&v| v < pivot_value);

    lesser_than_pivot = quick_sorted(lesser_than_pivot);
    lesser_than_pivot.push(pivot_value);
    lesser_than_pivot.extend(quick_sorted(greater_than_pivot));
    lesser_than_pivot
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_quicksort::quick_sorted;

    #[test]
    fn should_return_sorted_slice() {
        let my_vec = vec![1, 51512, 7, 4, 23, 45, 7, 8];

        let sorted_vec = quick_sorted(my_vec);

        assert_eq!(sorted_vec, vec![1, 4, 7, 7, 8, 23, 45, 51512])
    }
}
