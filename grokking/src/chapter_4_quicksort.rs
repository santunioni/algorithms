/// Partitions an array and return the index at which it is partitioned
/// All values before the returned index are lesser than the value in the index
/// All values after the returned index are greater than the value in the index
///
/// # Returns
/// A tuple of two arrays. It is guaranteed the all items in first array are lesser
/// than all items in second array.
fn partition_array<T>(slice: &mut [T]) -> (&mut [T], &mut [T])
where
    T: PartialOrd + Copy,
{
    let length = slice.len();

    if length <= 2 {
        return if length == 2 {
            if slice[0] > slice[1] {
                slice.swap(0, 1)
            }
            let (first_half, second_half) = slice.split_at_mut(1);
            (first_half, &mut second_half[1..])
        } else {
            (slice, &mut [])
        };
    }

    let pivot_idx = length - 1;
    let pivot_value = slice[pivot_idx];

    let mut lower_idx = 0;
    let mut higher_idx = pivot_idx - 1;

    while lower_idx < higher_idx {
        while slice[lower_idx] < pivot_value {
            lower_idx += 1;
        }

        while slice[higher_idx] > pivot_value && higher_idx > 0 {
            higher_idx -= 1;
        }

        if lower_idx >= higher_idx {
            break;
        }

        slice.swap(lower_idx, higher_idx)
    }

    let new_pivot_idx = lower_idx;
    slice.swap(new_pivot_idx, pivot_idx);

    let (first_half, second_half) = slice.split_at_mut(new_pivot_idx);
    (first_half, &mut second_half[1..])
}

/// A tail recursive version of quicksort, subject to compiler optimization.
/// It will carry an array of yet unsorted slices to further calls to itself.
fn quick_sorted_tailed<T>(slices_to_sort: Vec<&mut [T]>)
where
    T: PartialOrd + Copy,
{
    let slices_yet_to_sort: Vec<&mut [T]> = slices_to_sort
        .into_iter()
        // Could be easily parallelized for larger datasets, thanks to functional style.
        // But for smaller datasets, the overhead of context switching between threads
        // isn't compensated by parallelism in computation.
        .flat_map(|slice| {
            let (lesser_half, greater_half) = partition_array(slice);
            vec![lesser_half, greater_half]
        })
        .filter(|slice| slice.len() >= 2)
        // Slice with 2 or more items need sorting
        .collect();

    if slices_yet_to_sort.len() == 0 {
        return;
    }

    quick_sorted_tailed(slices_yet_to_sort)
}

fn quick_sorted_vec<T>(vec: &mut Vec<T>)
where
    T: PartialOrd + Copy,
{
    quick_sorted_tailed(vec![vec.as_mut_slice()])
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
        let mut my_vec: Vec<u32> = (0..3000).rev().collect();
        quick_sorted_vec(&mut my_vec);
        assert_eq!(my_vec, (0..3000).collect::<Vec<u32>>())
    }

    #[test]
    fn should_partition_removing_pivot() {
        let mut my_vec = vec![1, 51512, 7, 4, 23, 45, 7, 8];
        let (first_half, second_half) = partition_array(&mut my_vec);

        assert_eq!(first_half, [1, 7, 7, 4]);
        assert_eq!(second_half, [45, 51512, 23]);
    }

    #[test]
    fn should_partition_slice_of_two_numbers() {
        let mut my_vec = vec![1, 2];
        let result = partition_array(&mut my_vec);

        assert_eq!(result, (&mut [1][..], &mut [][..]));
    }

    #[test]
    fn should_partition_slice_of_three_numbers() {
        let mut my_vec = vec![2, 1, 3];
        let result = partition_array(&mut my_vec);

        assert_eq!(result, (&mut [2, 1][..], &mut [][..]));
    }
}
