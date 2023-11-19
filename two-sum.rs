
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        'process: loop {
            for i in 0..nums.len() {
                for j in i+1..nums.len() {
                    if i != j && nums[i] + nums[j] == target {
                        res.push(i as i32);
                        res.push(j as i32);
                        break 'process;
                    }
                }
            }
        }

        res
    }
}
