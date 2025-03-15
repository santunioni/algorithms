fn quick_sorted<T>(vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    fn split_at_pivot<T>(vec_to_split: Vec<T>) -> (Vec<T>, (usize, T), Vec<T>)
    where
        T: PartialOrd + Copy,
    {
        let pivot_index = vec_to_split.len() / 2;
        let pivot_value = vec_to_split[pivot_index];

        let (lesser_than_pivot, greater_than_pivot) = vec_to_split
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i != pivot_index)
            .map(|(_, v)| v)
            .partition(|&v| v < pivot_value);

        (
            lesser_than_pivot,
            (pivot_index, pivot_value),
            greater_than_pivot,
        )
    }

    if vec.len() < 2 {
        return vec;
    };

    let (lesser_than_pivot, (_, pivot_value), greater_than_pivot) = split_at_pivot(vec);

    quick_sorted(lesser_than_pivot)
        .into_iter()
        .chain(std::iter::once(pivot_value))
        .chain(quick_sorted(greater_than_pivot).into_iter())
        .collect()
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
