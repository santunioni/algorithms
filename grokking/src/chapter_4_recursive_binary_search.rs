fn recursive_binary_search<T: PartialOrd>(list: &[T], searched_value: &T) -> Option<usize> {
    fn tail_recursive_binary_search<T: PartialOrd>(
        list: &[T],
        searched_value: &T,
        start_index: usize,
    ) -> Option<usize> {
        let mid_index = list.len() / 2;
        let mid_value: &T = &list[mid_index];

        let (list, start_index) = {
            let (first_half, second_half) = list.split_at(mid_index);
            if searched_value < mid_value {
                (first_half, start_index)
            } else if searched_value > mid_value {
                if mid_index == 0 {
                    return None;
                }
                (second_half, start_index + mid_index)
            } else {
                return Some(start_index + mid_index);
            }
        };

        tail_recursive_binary_search(list, searched_value, start_index)
    }

    tail_recursive_binary_search(list, searched_value, 0)
}

fn loop_binary_search<T: PartialOrd>(list: &[T], searched_value: &T) -> Option<usize> {
    let mut start_index = 0;
    let mut list = list;

    loop {
        let mid_index = list.len() / 2;
        let mid_value: &T = &list[mid_index];

        let (first_half, second_half) = list.split_at(mid_index);
        if searched_value < mid_value {
            list = first_half;
        } else if searched_value > mid_value {
            if mid_index == 0 {
                return None;
            }
            list = second_half;
            start_index += mid_index;
        } else {
            return Some(start_index + mid_index);
        }

        if list.is_empty() {
            break;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_recursive_binary_search::{loop_binary_search, recursive_binary_search};

    #[test]
    fn should_return_found_value_recursive() {
        let my_ordered_array: Vec<u32> = (0..10u32.pow(6)).collect();
        assert_eq!(recursive_binary_search(&my_ordered_array, &1037u32), Some(1037usize));
    }

    #[test]
    fn should_not_return_found_value_recursive() {
        let my_ordered_array = vec![0, 1, 2, 3, 5, 6, 7, 8, 9];
        assert_eq!(recursive_binary_search(&my_ordered_array, &4), None);
    }

    #[test]
    fn should_return_found_value_loop() {
        let my_ordered_array: Vec<u32> = (0..10u32.pow(6)).collect();
        assert_eq!(loop_binary_search(&my_ordered_array, &1037u32), Some(1037usize));
    }

    #[test]
    fn should_not_return_found_value_loop() {
        let my_ordered_array = vec![0, 1, 2, 3, 5, 6, 7, 8, 9];
        assert_eq!(loop_binary_search(&my_ordered_array, &4), None);
    }
}
