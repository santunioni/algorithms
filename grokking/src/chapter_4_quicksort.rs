/// Partitions an array and return the index at which it is partitioned
/// All values before the returned index are lesser than the value in the index
/// All values after the returned index are greater than the value in the index
///
/// # Returns
/// A tuple of two arrays. It is guaranteed the all items in first array are lesser
/// than all items in second array.
fn partition_slice<T>(slice: &mut [T]) -> [&mut [T]; 2]
where
    T: PartialOrd,
{
    let length = slice.len();

    if length <= 2 {
        if length == 2 && slice[0] > slice[1] {
            slice.swap(0, 1)
        }
        return [&mut [], &mut []];
    }

    let pivot_idx = length / 2;
    let pivot_raw_ptr = &raw mut slice[pivot_idx];

    let mut lower_index = 0;
    let mut higher_index = length - 1;

    while lower_index < higher_index {
        while unsafe { slice[lower_index] < *pivot_raw_ptr } || lower_index == pivot_idx {
            lower_index += 1;
        }

        while (unsafe { slice[higher_index] > *pivot_raw_ptr } && higher_index > 0)
            || higher_index == pivot_idx
        {
            higher_index -= 1;
        }

        if lower_index >= higher_index {
            break;
        }

        slice.swap(lower_index, higher_index)
    }

    let displaced_pivot_index = if lower_index < pivot_idx {
        slice.swap(lower_index, pivot_idx);
        lower_index
    } else if higher_index > pivot_idx {
        slice.swap(higher_index, pivot_idx);
        higher_index
    } else {
        pivot_idx
    };

    let (first_half, second_half) = slice.split_at_mut(displaced_pivot_index);
    let second_half_without_pivot = &mut second_half[1..];
    [first_half, second_half_without_pivot]
}

fn filter_slices_yet_to_sort<T: PartialOrd>(slices_to_sort: Vec<&mut [T]>) -> Vec<&mut [T]> {
    slices_to_sort
        .into_iter()
        // Could be easily parallelized for larger datasets, thanks to functional style.
        // But for smaller datasets, the overhead of context switching between threads
        // isn't compensated by parallelism in computation.
        .flat_map(partition_slice)
        // Only slices with 2 or more items need partitioning
        .filter(|slice| slice.len() >= 2)
        .collect()
}

/// A loop version of quicksort, the fastest possible
fn quick_sorted_loop<T: PartialOrd>(mut slices_to_sort: Vec<&mut [T]>) {
    while !slices_to_sort.is_empty() {
        slices_to_sort = filter_slices_yet_to_sort(slices_to_sort);
    }
}

/// A tail recursive version of quicksort, subject to compiler optimization.
/// It will carry an array of yet unsorted slices to further calls to itself.
/// A tail recursive version of quicksort, subject to compiler optimization.
/// It will carry an array of yet unsorted slices to further calls to itself.
///
/// # Panics
/// This is stack overflowing with large values because Rust Compiler doesn't implement
/// tail calls optimization. Happyly, transforming to loop is easy after I figure out
/// how to implement as tail call.
fn quick_sorted_tailed<T: PartialOrd>(mut slices_yet_to_sort: Vec<&mut [T]>) {
    slices_yet_to_sort = filter_slices_yet_to_sort(slices_yet_to_sort);
    if slices_yet_to_sort.is_empty() {
        return;
    }
    quick_sorted_tailed(slices_yet_to_sort);
}

fn quick_sorted_vec<T: PartialOrd>(vec: &mut Vec<T>) {
    quick_sorted_loop(vec![vec.as_mut_slice()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_array_in_place() {
        let mut my_vec = vec![1, 51512, 7, 4, 23, 45, 7, 8];
        quick_sorted_vec(&mut my_vec);
        assert_eq!(my_vec, vec![1, 4, 7, 7, 8, 23, 45, 51512])
    }

    #[test]
    fn should_sort_big_array_in_place() {
        let mut my_vec = (0..3000000).rev().collect::<Vec<u32>>();
        quick_sorted_vec(&mut my_vec);
        assert_eq!(my_vec, (0..3000000).collect::<Vec<u32>>())
    }

    #[test]
    fn should_partition_removing_pivot() {
        let mut my_vec = vec![1, 51512, 7, 4, 23, 45, 7, 8];
        let [first_half, second_half] = partition_slice(&mut my_vec);

        assert_eq!(first_half, [1, 8, 7, 4, 7]);
        assert_eq!(second_half, [45, 51512]);
    }

    #[test]
    fn should_partition_slice_of_two_numbers() {
        let mut my_vec = vec![1, 2];
        let [first_half, second_half] = partition_slice(&mut my_vec);

        assert_eq!(first_half, []);
        assert_eq!(second_half, []);
    }

    #[test]
    fn should_partition_slice_of_three_numbers() {
        let mut my_vec = vec![2, 1, 3];
        let [first_half, second_half] = partition_slice(&mut my_vec);

        assert_eq!(first_half, []);
        assert_eq!(second_half, [2, 3]);
    }
}
