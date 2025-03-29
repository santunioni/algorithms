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

fn copy_items_to_selection_sorted<T>(vec: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut copied_vec: Vec<T> = vec.to_vec();
    transfer_ordered_values_to_new_vector!(copied_vec)
}

fn borrow_items_to_selection_sorted<T>(vec: &[T]) -> Vec<&T>
where
    T: PartialOrd,
{
    let mut copied_vec: Vec<&T> = vec.iter().collect();
    transfer_ordered_values_to_new_vector!(copied_vec)
}

fn move_items_to_selection_sorted<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd,
{
    transfer_ordered_values_to_new_vector!(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_sorted_array_from_reference() {
        assert_eq!(
            copy_items_to_selection_sorted(&[2, 1, -30]),
            vec![-30, 1, 2]
        );
    }

    #[test]
    fn should_return_sorted_array_taking_ownership() {
        assert_eq!(
            move_items_to_selection_sorted(vec![2, 1, -30]),
            vec![-30, 1, 2]
        );
    }

    #[test]
    fn should_return_refs_to_owned_sorted_values() {
        let unsorted_vec = vec![Box::new(2), Box::new(1), Box::new(-30)];
        let expected_sorted_vec = [Box::new(-30), Box::new(1), Box::new(2)];

        assert_eq!(
            borrow_items_to_selection_sorted(&unsorted_vec)
                .into_iter()
                .collect::<Vec<&Box<i32>>>(),
            expected_sorted_vec.iter().collect::<Vec<&Box<i32>>>()
        );
    }

    #[test]
    fn should_return_refs_to_unowned_sorted_values() {
        let minus_thirty = Box::new(-30);
        let one = Box::new(1);
        let two = Box::new(2);

        let actual_sorted: Vec<&Box<i32>> =
            copy_items_to_selection_sorted(&[&two, &one, &minus_thirty]);

        let expected = vec![&minus_thirty, &one, &two];
        assert_eq!(actual_sorted, expected);

        assert_eq!(format!("{:p}", &minus_thirty), format!("{:p}", expected[0]),); // Assert they refer to same object
        assert_eq!(
            format!("{:p}", &minus_thirty),
            format!("{:p}", actual_sorted[0]),
        ); // Assert they refer to same object
    }

    #[test]
    fn it_sorts_boxes_moved() {
        assert_eq!(
            move_items_to_selection_sorted(vec![Box::new(2), Box::new(1), Box::new(-30)]),
            vec![Box::new(-30), Box::new(1), Box::new(2)]
        );
    }
}
