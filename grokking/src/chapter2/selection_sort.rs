trait WithSelectionSort {
    fn selection_sorted(self: &Self) -> Self;
}

impl<T> WithSelectionSort for Vec<T>
where
    T: PartialOrd + Copy,
{
    fn selection_sorted(self: &Self) -> Self {
        let mut copied_vec: Vec<T> = self.iter().map(|x| *x).collect();
        let mut new_vec = Vec::with_capacity(self.capacity());

        while copied_vec.len() > 0 {
            let mut curr_min_index = 0;

            for check_max_index in 0..copied_vec.len() {
                let check_max_value = copied_vec[check_max_index];
                if check_max_value < copied_vec[curr_min_index] {
                    curr_min_index = check_max_index;
                }
            }

            let max_value = copied_vec.remove(curr_min_index);
            new_vec.push(max_value);
        }

        new_vec
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sorts_integers() {
        assert_eq!(vec![2, 1, -30].selection_sorted(), vec![-30, 1, 2]);
    }
}
