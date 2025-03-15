fn binary_search<T: PartialOrd>(list: &[T], searched_value: &T) -> usize {
    fn tail_recursive_binary_search<T: PartialOrd>(
        list: &[T],
        searched_value: &T,
        start_index: usize,
    ) -> usize {
        let mid_index = list.len() / 2;
        let mid_value: &T = &list[mid_index];

        let (list, start_index) = {
            let (first_half, second_half) = list.split_at(mid_index);
            if searched_value < mid_value {
                (first_half, start_index)
            } else if searched_value > mid_value {
                (second_half, start_index + mid_index)
            } else {
                return start_index + mid_index;
            }
        };

        tail_recursive_binary_search(list, searched_value, start_index)
    }

    tail_recursive_binary_search(list, searched_value, 0)
}

#[cfg(test)]
mod tests {
    use crate::chapter_4_recursive_binary_search::binary_search;

    #[test]
    fn should_return_found_value() {
        let my_ordered_array: Vec<u32> = (0..10u32.pow(6)).collect();
        assert_eq!(binary_search(&my_ordered_array, &1037u32), 1037usize);
    }
}
