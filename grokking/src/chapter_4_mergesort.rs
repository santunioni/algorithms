fn merge_vectors<T: PartialOrd>(mut first_half: Vec<T>, mut second_half: Vec<T>) -> Vec<T> {
    let mut new_array: Vec<T> = Vec::with_capacity(first_half.len() + second_half.len());

    let mut first_half = first_half.drain(0..);
    let mut second_half = second_half.drain(0..);

    let mut option_from_first_half = first_half.next();
    let mut option_from_second_half = second_half.next();
    loop {
        match (
            option_from_first_half.take(),
            option_from_second_half.take(),
        ) {
            (Some(item_from_first_half), Some(item_from_second_half)) => {
                if item_from_first_half < item_from_second_half {
                    new_array.push(item_from_first_half);
                    option_from_first_half = first_half.next();
                    option_from_second_half = Some(item_from_second_half);
                } else {
                    new_array.push(item_from_second_half);
                    option_from_first_half = Some(item_from_first_half);
                    option_from_second_half = second_half.next();
                }
            }
            (Some(item_from_first_half), None) => {
                new_array.push(item_from_first_half);
                option_from_first_half = first_half.next();
            }
            (None, Some(item_from_second_half)) => {
                new_array.push(item_from_second_half);
                option_from_second_half = second_half.next();
            }
            (None, None) => break,
        }
    }

    new_array
}

fn split_vector<T>(mut vec: Vec<T>) -> (Vec<T>, Vec<T>) {
    let first_half: Vec<T> = vec.drain(0..vec.len() / 2).collect();
    let second_half: Vec<T> = vec.drain(0..).collect();
    (first_half, second_half)
}

fn merged_sorted_vec<T: PartialOrd>(vec: Vec<T>) -> Vec<T> {
    if vec.len() == 1 {
        return vec;
    }

    let (first_half, second_half) = split_vector(vec);

    merge_vectors(
        merged_sorted_vec(first_half),
        merged_sorted_vec(second_half),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_array_in_place() {
        let my_vec = vec![1, 51512, 7, 4, 23, 45, 7, 8];
        let my_vec = merged_sorted_vec(my_vec);
        assert_eq!(my_vec, vec![1, 4, 7, 7, 8, 23, 45, 51512])
    }

    #[test]
    fn should_sort_big_array_in_place() {
        let my_vec = (0..3000000).rev().collect::<Vec<u32>>();
        let my_vec = merged_sorted_vec(my_vec);
        assert_eq!(my_vec, (0..3000000).collect::<Vec<u32>>())
    }

    #[test]
    fn should_merge_sorted_alternating_vectors() {
        let vec_odd = vec![1, 3, 5, 7, 9];
        let vec_even = vec![2, 4, 6, 8, 10];

        assert_eq!(
            merge_vectors(vec_odd, vec_even),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }

    #[test]
    fn should_merge_sorted_ordered_vectors() {
        let vec_odd = vec![1, 2, 3, 4, 5];
        let vec_even = vec![6, 7, 8, 9, 10];

        assert_eq!(
            merge_vectors(vec_odd, vec_even),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }
}
