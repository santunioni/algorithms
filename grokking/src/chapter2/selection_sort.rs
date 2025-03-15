macro_rules! transfer_ordered_values_to_new_vector {
    ($owned_vector:ident) => {{
        let mut new_vec = Vec::with_capacity($owned_vector.capacity());
        while !$owned_vector.is_empty() {
            let mut current_min_index = 0;

            for checked_min_index in 0..$owned_vector.len() {
                if $owned_vector[checked_min_index] < $owned_vector[current_min_index] {
                    current_min_index = checked_min_index;
                }
            }

            let min_value = $owned_vector.remove(current_min_index);
            new_vec.push(min_value);
        }
        new_vec
    }};
}

fn selection_sorted_copied<T>(vec: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut copied_vec: Vec<T> = vec.iter().map(|x| *x).collect();
    transfer_ordered_values_to_new_vector!(copied_vec)
}

fn selection_sorted_ref<T>(vec: &Vec<T>) -> Vec<&T>
where
    T: PartialOrd,
{
    let mut copied_vec: Vec<&T> = vec.iter().collect();
    transfer_ordered_values_to_new_vector!(copied_vec)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_integers() {
        assert_eq!(selection_sorted_copied(&vec![2, 1, -30]), vec![-30, 1, 2]);
    }

    #[test]
    fn it_sorts_boxes() {
        let unsorted_vec = vec![Box::new(2), Box::new(1), Box::new(-30)];
        let expected_sorted_vec = vec![Box::new(-30), Box::new(1), Box::new(2)];
        let expected_sorted_vec_pointers: Vec<&Box<i32>> = expected_sorted_vec.iter().collect();

        let sorted_vec: Vec<&Box<i32>> = selection_sorted_ref(&unsorted_vec).into_iter().collect();

        assert_eq!(sorted_vec, expected_sorted_vec_pointers);
    }
}
