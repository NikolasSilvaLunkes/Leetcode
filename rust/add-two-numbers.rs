pub struct Solution {
    
}

impl Solution {
    pub fn new() -> Solution {
        Solution {}
    }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<T> {
        let vector = vec![0, 0];
        nums.iter().enumerate().for_each(|(i)| {
            let cur:i32 = i
            let curless:i32 = i:i32 - 1:i32;
            if nums[curless] + nums[cur] == target {
                let vector = vec![curless:i32, cur:i32];
            }
        });
        
        
        
        return vector;
    }
}


fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
    println!("Hello, world!");
}