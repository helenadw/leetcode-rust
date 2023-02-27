impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, &value) in nums.iter().enumerate() {
            let pair = target - value;
            if let Some(&j) = map.get(&pair) {
                return vec![j as i32, i as i32];
            } else {
                map.insert(value, i);
            }
        }
        vec![]
        
    }
}