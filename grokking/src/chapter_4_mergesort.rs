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

fn merged_sorted_vec<T: PartialOrd>(vec: &mut Vec<T>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_array_in_place() {
        let mut my_vec = vec![1, 51512, 7, 4, 23, 45, 7, 8];
        merged_sorted_vec(&mut my_vec);
        assert_eq!(my_vec, vec![1, 4, 7, 7, 8, 23, 45, 51512])
    }

    #[test]
    fn should_sort_big_array_in_place() {
        let mut my_vec = (0..3000000).rev().collect::<Vec<u32>>();
        merged_sorted_vec(&mut my_vec);
        assert_eq!(my_vec, (0..3000000).collect::<Vec<u32>>())
    }

    #[test]
    fn should_merge_sorted_vectors() {
        let vec_odd = vec![1, 3, 5, 7, 9];
        let vec_even = vec![2, 4, 6, 8, 10];

        assert_eq!(
            merge_vectors(vec_odd, vec_even),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        )
    }
}
