struct Solution{
    nums : Vec<i32>,
    target: i32

}

impl Solution {
    
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vec_number_a:usize = 0;
        let mut vec_umber_b:usize = 0;
        for _ in &nums{
            vec_umber_b = 0;
            for _ in &nums{
                if vec_umber_b == vec_number_a{
                    break;
                }
                if nums[vec_number_a]+nums[vec_umber_b]==target{
                    return vec![vec_umber_b as i32,vec_number_a as i32];
                }
                vec_umber_b += 1;
            }
            vec_number_a+=1;
        }


        return vec![];
    }

}



fn main() {
    let nums = vec![3,3];
    let target = 6;
    let answer:Vec<i32>;
    answer = Solution::two_sum(nums, target);
    println!("{:?}",answer)
    
}