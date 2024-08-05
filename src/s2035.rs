use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut counter = HashMap::new();

        // Count occurrences of each item
        for item in arr.iter() {
            *counter.entry(item).or_insert(0) += 1;
        }

        let mut count = 0;

        // Find the k-th distinct element
        for item in arr.iter() {
            if counter[item] == 1 {
                count += 1;
                if count == k {
                    return item.into();
                }
            }
        }

        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            "a".to_string(),
            Solution::kth_distinct(
                vec![
                    "d".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "c".to_string(),
                    "a".to_string()
                ],
                2
            )
        );
        assert_eq!(
            "aaa".to_string(),
            Solution::kth_distinct(
                vec![
                    "aaa".to_string(),
                    "aa".to_string(),
                    "a".to_string(),
                ],
                1
            )
        );
        assert_eq!(
            "".to_string(),
            Solution::kth_distinct(
                vec![
                    "a".to_string(),
                    "b".to_string(),
                    "a".to_string(),
                ],
                3
            )
        );
    }
}
