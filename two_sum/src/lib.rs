use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = [3, 2, 4];
        let target = 6;

        assert_eq!(crate::two_sum(input.to_vec(), target), [1, 2]);
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::new();
    for (index, number) in nums.iter().enumerate() {
        if !hash_map.contains_key(&number) {
            hash_map.insert(number, index);
        }

        let difference = target - number;

        match hash_map.get(&difference) {
            Some(diff_index) => {
                if *diff_index != index {
                    return [*diff_index as i32, index as i32].into();
                }
            }
            None => continue,
        }
    }

    [].into()
}
