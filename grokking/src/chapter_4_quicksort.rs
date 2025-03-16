/// Partitions an array and return the index at which it is partitioned
/// All values before the returned index are lesser than the value in the index
/// All values after the returned index are greater than the value in the index
fn partition_array<T>(slice: &mut [T]) -> (&mut [T], &mut [T])
where
    T: PartialOrd + Copy,
{
    let length = slice.len();

    if length < 2 {
        return (slice, &mut []);
    }

    let pivot_idx = length - 1;
    let pivot_value = slice[pivot_idx];

    let mut lower_idx = 0;
    let mut higher_idx = pivot_idx - 1;

    while lower_idx < higher_idx {
        while slice[lower_idx] < pivot_value && lower_idx < higher_idx {
            lower_idx += 1;
        }

        while slice[higher_idx] > pivot_value && higher_idx > lower_idx {
            higher_idx -= 1;
        }

        if lower_idx >= higher_idx {
            break;
        }

        slice.swap(lower_idx, higher_idx)
    }

    slice.swap(lower_idx, pivot_idx);

    let (first_half, second_half) = slice.split_at_mut(lower_idx);
    (first_half, &mut second_half[1..])
}

// fn quick_sorted_tailed<T>(mut array_of_slices: Vec<&mut [T]>)
// where
//     T: PartialOrd + Copy,
// {
//     let (array_of_slices, should_call_again) = array_of_slices
//         .into_iter()
//         .fold(
//             (Vec::new(), false),
//             |(array_of_slices, should_call_again), slice| {
//
//                 if slice.len() < 2 {
//                     array_of_slices;
//                 };
//             }
//         )
//         .map(|slice| {
//             let (first_half, second_half) = partition_array(slice.as_mut());
//         })
//         .collect()
// }

fn quick_sorted_slice<T>(slice: &mut [T])
where
    T: PartialOrd + Copy,
{
    if slice.len() < 2 {
        return;
    };

    let (first_half, second_half) = partition_array(slice.as_mut());

    quick_sorted_slice(first_half);
    quick_sorted_slice(second_half);
}

fn quick_sorted_vec<T>(vec: &mut Vec<T>)
where
    T: PartialOrd + Copy,
{
    quick_sorted_slice(vec.as_mut_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_slice() {
        let mut my_vec = vec![1, 51512, 7, 4, 23, 45, 7, 8];
        quick_sorted_vec(&mut my_vec);
        assert_eq!(my_vec, vec![1, 4, 7, 7, 8, 23, 45, 51512])
    }
}
