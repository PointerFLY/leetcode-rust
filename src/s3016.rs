struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counter: [usize; 26] = [0usize; 26];
        for c in word.bytes() {
            let idx = (c - 'a' as u8) as usize;
            counter[idx as usize] += 1
        }
        counter.sort();
        
        let mut total: usize = 0;
        for (i, v) in counter.into_iter().rev().enumerate() {
            total += (i / 8 + 1) * v;
        }

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::minimum_pushes("abcde".into()), 5);
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".into()), 12);
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".into()),
            24
        );
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiiiyyyyy".into()),
            31
        );
    }
}
